use clap::{Arg, App, SubCommand};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use crate::utils::git_utils;

const CRANE_FILE: &str = ".crane";

// load the python format file .crane and parse the dict "solutions" in it
pub fn load_solutions() ->  HashMap<String, String>{
    pyo3::prepare_freethreaded_python();
    // evaluate the python file CRANE_FILE and return the dict "solutions"
    Python::with_gil(|py| {
        // Load the python code from the file CRANE_FILE
        let py_content = std::fs::read_to_string(CRANE_FILE).unwrap();

        // Evaluate the python code and load the dict "solutions"
        let locals = PyDict::new(py);
        py.run(py_content.as_str(), None, Some(locals)).unwrap();

        // Read the dict "solutions"
        let py_solutions = locals.get_item("solutions").unwrap();

        // Convert the dict "solutions" to a HashMap
        let solutions: HashMap<String, String> = py_solutions.extract().unwrap();
        println!("{:?}", solutions);

        solutions
    })
}


pub fn create_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("sync")
        .about("Syncs the local repository with the remote repository")
        .arg(Arg::with_name("force")
             .short("f")
             .long("force")
             .help("Force sync even if there are local changes")
             .takes_value(true))
}

pub fn run(matches: &clap::ArgMatches) {
    println!("{:?}", matches);
    // Check if current directory is in a git repository
    if !git_utils::is_git_repo("") {
        println!("Not a git repository");
        return;
    }
    load_solutions();
}
