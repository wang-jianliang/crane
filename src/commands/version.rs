use clap::{App, SubCommand};

pub fn create_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("version")
        .about("Print the version of the application")
}

pub fn run(_matches: &clap::ArgMatches) {
    let version = get_version();
    println!("{}", version);
}

// read version from cargo.toml
pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    version.to_string()
}
