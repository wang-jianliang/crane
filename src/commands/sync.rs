use clap::Args;
use exitcode;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::PathBuf;
use std::process;

use crate::components::component::Attributes;
use crate::components::solution::Solution;
use crate::constants::CRANE_FILE;
use crate::utils::git_utils;

// load the python format file .crane and parse the dict "solutions" in it
pub fn load_solutions() {
    pyo3::prepare_freethreaded_python();
    // evaluate the python file CRANE_FILE and return the dict "solutions"
    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, &std::fs::read_to_string(CRANE_FILE).unwrap(), "", "").unwrap();

        // 获取Python变量"solutions"
        let solutions: &PyList = module.getattr("solutions").unwrap().downcast().unwrap();

        // 将Python变量转换为Rust结构体
        let mut result = vec![];

        for solution in solutions.iter() {
            let path = solution
                .get_item("path")
                .unwrap()
                .extract::<String>()
                .unwrap();
            let deps_file = solution
                .get_item("deps_file")
                .unwrap()
                .extract::<String>()
                .unwrap();
            let url = solution
                .get_item("url")
                .unwrap()
                .extract::<String>()
                .unwrap();

            let s = Solution {
                comp_attrs: Attributes {
                    path,
                    deps_file,
                    url,
                },
            };
            result.push(s);
        }

        // 输出最终结果
        println!("{:#?}", result);
    })
}

#[derive(Args, Debug)]
pub struct SyncArgs {
    pub dir: Option<PathBuf>,
}

fn run_sync(target_dir: &PathBuf) {
    println!("Sync dependencies in {:?}", target_dir);
    // Check if current directory is in a git repository
    if !git_utils::is_git_repo(Some(target_dir)) {
        println!("Directory {:?} is not a git repository", target_dir);
        process::exit(exitcode::DATAERR);
    }
    load_solutions();
}

pub fn run(args: &SyncArgs) {
    println!("{:?}", args);

    if let Some(target_dir) = &args.dir {
        run_sync(target_dir);
    } else {
        println!("Syncing current directory");
        run_sync(&PathBuf::from("."));
    }
}
