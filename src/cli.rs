use clap::{Parser, Subcommand};

use crate::commands::*;
use crate::errors::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Sync(sync::SyncArgs),
}

pub async fn run_command(cmd: &Command) -> Result<(), Error> {
    match cmd {
        Command::Sync(args) => sync::run(args).await,
    }
}
