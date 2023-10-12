use crate::errors::Error;
use clap::Args;

use std::path::PathBuf;

use crate::components::component::walk_components;
use crate::constants::CRANE_FILE;
use crate::visitors::sync_visitor::ComponentSyncVisitor;

#[derive(Args, Debug)]
pub struct CommandArgs {
    pub dir: Option<PathBuf>,
}

pub async fn run(args: &CommandArgs) -> Result<(), Error> {
    let visitor = ComponentSyncVisitor::new();
    let deps_file = PathBuf::from(CRANE_FILE);
    if let Some(target_dir) = &args.dir {
        println!("Sync in directory {:?}", target_dir);
        if let Err(err) = walk_components(&visitor, target_dir, &deps_file).await {
            return Err(err);
        }
        Ok(())
    } else {
        println!("Syncing current directory");
        if let Err(err) = walk_components(&visitor, &PathBuf::from("."), &deps_file).await {
            return Err(err);
        }
        Ok(())
    }
}
