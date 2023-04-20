use clap::{Arg, App, SubCommand};
mod package_create;
mod package_list;

pub fn create_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("package")
        .about("Manage packages")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![
            // command to list packages
            SubCommand::with_name("list")
            .about("List packages")
            .arg(Arg::with_name("count").short("c").long("count").help("Max number of packages to list")),
            // command to create a new package, argument to specify the configuration file of the package
            SubCommand::with_name("create")
            .about("Create a new package")
            .arg(Arg::with_name("config").short("c").long("config").help("Configuration file of the package")),
        ])
}

pub fn run(matches: &clap::ArgMatches) {
    // TODO: Implement this function
    println!("{:?}", matches);
    // Call the appropriate command function based on the subcommand
    match matches.subcommand() {
        ("list", Some(matches)) => { package_list::run(matches); },
        ("create", Some(matches)) => { package_create::run(matches); }
        _ => println!("Invalid command"),
        // The `_` case handles errors and should never be reached
    }
}
