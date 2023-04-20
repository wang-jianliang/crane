use clap::{Arg, App};

mod commands;
mod utils;

fn main() {
    let matches = App::new("crane")
        .version("1.0")
        .author("jianliangw1@gmail.com")
        .about("A command line tool for managing your project")
        .subcommand(commands::version::create_subcommand())
        .subcommand(commands::sync::create_subcommand())
        .subcommand(commands::package::create_subcommand())
        // Add additional subcommands as needed
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .get_matches();
    
    // Call the appropriate command function based on the subcommand
    match matches.subcommand() {
        ("sync", Some(sub_args)) => commands::sync::run(sub_args),
        ("version", Some(sub_args)) => commands::version::run(sub_args),
        ("package", Some(sub_args)) => commands::package::run(sub_args),
        // Add additional subcommands as needed
        _ => println!("Invalid command"),
    }
}
