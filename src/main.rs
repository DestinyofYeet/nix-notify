use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use clap::Parser;
use django_rs::{
    server::{DjangoServer, database_strategy::default_strategies::SqliteStrategy},
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};
use signal_hook::{
    consts::{SIGHUP, TERM_SIGNALS},
    flag,
    iterator::{SignalsInfo, exfiltrator::WithOrigin},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::args::Args;

pub mod args;
pub mod rss;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let level = format!(
        "{},reqwest=info,h2=info,hyper_util=info,rustls_platform_verifier=info",
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

    let server = DjangoServer::new(8, TracingStrategy {}, SqliteStrategy::new_memory())?;

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
