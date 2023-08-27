use crate::errors::Error;
use clap::Args;

use std::path::PathBuf;

use crate::components::component::walk_components;
use crate::constants::CRANE_FILE;
use crate::utils::parser;
use crate::visitors::sync_visitor::ComponentSyncVisitor;

#[derive(Args, Debug)]
pub struct SyncArgs {
    pub dir: Option<PathBuf>,
}

async fn run_sync(root_dir: &PathBuf) -> Result<(), Error> {
    println!("Sync dependencies in {:?}", root_dir);

    let full_path: PathBuf = root_dir.join(PathBuf::from(CRANE_FILE));
    let deps = parser::parse_components(&full_path, "deps")?;

    walk_components(deps, ComponentSyncVisitor::new(), &root_dir).await?;
    Ok(())
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
