use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short='v', action = clap::ArgAction::Count, help="Sets the verbose level. More v's more output", default_value="0")]
    pub verbose: u8,

    #[arg(short = 'c', help = "The path of the configuration file")]
    pub configuration: PathBuf,
}
