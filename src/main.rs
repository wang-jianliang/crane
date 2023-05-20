use std::env;

use clap::Parser;
use crane::cli::run_command;
use crane::cli::Cli;
use crane::constants::DEFAULT_LOG_LEVEL;

#[async_std::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", DEFAULT_LOG_LEVEL);
    }
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Some(cmd) => {
            run_command(cmd).await;
        }
        None => {
            panic!("No command provided")
        }
    }
}
