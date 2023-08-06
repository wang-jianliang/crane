use crate::errors::Error;
use clap::Args;
use exitcode;
use futures::future::try_join_all;
use std::path::PathBuf;
use std::process;

use crate::constants::CRANE_FILE;
use crate::utils::{git_utils, parser};
use crate::components::component::{ComponentArena, ComponentID};

#[derive(Args, Debug)]
pub struct SyncArgs {
    pub dir: Option<PathBuf>,
}

async fn sync_component(id: ComponentID) -> Result<(), Error> {
    let arena = ComponentArena::instance().lock().unwrap();
    let component = arena.get(id).unwrap();
    return component.sync()
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

    let mut futures = Vec::new();
    for solution_id in solutions.iter() {
        futures.push(sync_component(*solution_id));
    }

    match try_join_all(futures).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
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
