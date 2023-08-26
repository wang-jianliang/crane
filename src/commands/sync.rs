use crate::errors::Error;
use clap::Args;
use exitcode;
use futures::future::try_join_all;
use std::path::PathBuf;
use std::process;

use crate::components::component::{walk_components, ComponentArena, ComponentID};
use crate::constants::CRANE_FILE;
use crate::utils::{git_utils, parser};
use crate::visitors::component_sync_visitor::ComponentSyncVisitor;

#[derive(Args, Debug)]
pub struct SyncArgs {
    pub dir: Option<PathBuf>,
}

async fn run_sync(target_dir: &PathBuf) -> Result<(), Error> {
    println!("Sync dependencies in {:?}", target_dir);
    // Check if current directory is in a git repository
    if !git_utils::is_git_repo(Some(target_dir)) {
        println!("Directory {:?} is not a git repository", target_dir);
        process::exit(exitcode::DATAERR);
    }

    let mut full_path: PathBuf = target_dir.clone();
    full_path.push(CRANE_FILE);
    let solutions = parser::parse_components(&full_path, "solutions")?;

    walk_components(solutions, ComponentSyncVisitor::new()).await
}

pub async fn run(args: &SyncArgs) -> Result<(), Error> {
    println!("{:?}", args);

    if let Some(target_dir) = &args.dir {
        run_sync(target_dir).await
    } else {
        println!("Syncing current directory");
        run_sync(&PathBuf::from(".")).await
    }
}
