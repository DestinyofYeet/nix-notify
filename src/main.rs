use std::{
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

use clap::Parser;
use django_rs::{
    server::{
        DjangoServer,
        database_strategy::{DatabaseStrategy, default_strategies::SqliteStrategy},
    },
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};
use signal_hook::{
    consts::{SIGHUP, TERM_SIGNALS},
    flag,
    iterator::{SignalsInfo, exfiltrator::WithOrigin},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    args::Args,
    config::{Config, FeedKind, FeedSource},
    feed::{atomfeed::AtomFeed, feeditem::FeedItem},
};

pub mod args;
pub mod config;
pub mod feed;
pub mod processor;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let level = format!(
        "{},reqwest=info,h2=info,hyper_util=info,rustls_platform_verifier=info,django_rs::server::database_strategy=info",
        match args.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        }
    );

    let stop_signal = Arc::new(AtomicBool::new(false));

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&stop_signal))?;
        flag::register(*sig, Arc::clone(&stop_signal))?;
    }

    let mut sigs = Vec::from(TERM_SIGNALS);
    sigs.push(SIGHUP);
    let mut signals = SignalsInfo::<WithOrigin>::new(&sigs)?;

    if !args.configuration.exists() {
        return Err(anyhow::format_err!(
            "Configuration file {:?} does not exist!",
            args.configuration
        ));
    }

    let configuration: Config = {
        let content = std::fs::read_to_string(args.configuration)?;
        toml::from_str(&content)?
    };

    let path = {
        match &configuration.general.database_path.parent() {
            Some(parent) => {
                if parent.exists() {
                    configuration.general.database_path
                } else {
                    return Err(anyhow::format_err!(
                        "Parent folder of database path {:?} does not exist!",
                        parent
                    ));
                }
            }
            None => {
                if !configuration.general.database_path.exists() {
                    return Err(anyhow::format_err!(
                        "Database path {:?} does not exist!",
                        configuration.general.database_path
                    ));
                }

                configuration.general.database_path
            }
        }
    };

    let mut server = DjangoServer::new(
        8,
        TracingStrategy {},
        SqliteStrategy::new(path.to_string_lossy().to_string()),
    )?;

    server.get_database().migrate_model::<FeedItem>()?;

    for feed in configuration.feeds.iter() {
        match feed.source {
            FeedSource::Nixpkgs => match feed.kind {
                FeedKind::GithubAtom => {
                    server.get_task_handler().spawn_task(AtomFeed::new(
                        feed.name.clone(),
                        format!(
                            "https://github.com/NixOS/nixpkgs/commits/{}.atom",
                            feed.name
                        ),
                        Duration::from_mins(feed.delay_minutes),
                    ))?;
                }
                FeedKind::GithubApi => todo!(),
            },
            FeedSource::Custom => todo!(),
        }
    }

    for info in &mut signals {
        info!("Received signal {:?}", info);
        match info.signal {
            SIGHUP => {
                info!("Sighup")
            }
            _ => {
                info!("Stopping...");
                break;
            }
        }
    }

    server.shutdown()?;

    Ok(())
}
