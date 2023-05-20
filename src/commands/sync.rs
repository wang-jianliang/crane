use clap::Args;
use exitcode;
use futures::future::try_join_all;
use std::path::PathBuf;
use std::process;

use crate::constants::CRANE_FILE;
use crate::utils::{git_utils, parser};

#[derive(Args, Debug)]
pub struct SyncArgs {
    pub dir: Option<PathBuf>,
}

async fn run_sync(target_dir: &PathBuf) {
    println!("Sync dependencies in {:?}", target_dir);
    // Check if current directory is in a git repository
    if !git_utils::is_git_repo(Some(target_dir)) {
        println!("Directory {:?} is not a git repository", target_dir);
        process::exit(exitcode::DATAERR);
    }

    let mut full_path: PathBuf = target_dir.clone();
    full_path.push(CRANE_FILE);
    let solutions = parser::parse_components(&full_path, "solutions");

    let mut futures = Vec::new();
    for solution in solutions.iter() {
        futures.push(solution.sync());
    }

    try_join_all(futures).await;
}

pub async fn run(args: &SyncArgs) {
    println!("{:?}", args);

    if let Some(target_dir) = &args.dir {
        run_sync(target_dir).await;
    } else {
        println!("Syncing current directory");
        run_sync(&PathBuf::from(".")).await;
    }
}
