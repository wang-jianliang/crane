use std::env;
use std::process;

use clap::Parser;
use crane::cli::run_command;
use crane::cli::Cli;
use crane::constants::CRANE_DEBUG;
use crane::constants::DEFAULT_LOG_LEVEL;
use crane::errors::Error;

fn exit_with_error(err: &Error) {
    println!("{}", err);
    process::exit(1);
}

fn exit_with_message(msg: &str) {
    println!("{}", msg);
    process::exit(1);
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", DEFAULT_LOG_LEVEL);
    }
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Some(cmd) => {
            let result = run_command(cmd).await;
            match &result {
                Ok(_) => {}
                Err(err) => {
                    if *CRANE_DEBUG {
                        panic!("{}", err);
                    } else {
                        exit_with_error(&err);
                    }
                }
            }
        }
        None => exit_with_message("No command provided"),
    }
}
