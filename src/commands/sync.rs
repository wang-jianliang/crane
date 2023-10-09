use crate::errors::Error;
use crate::visitors::component_visitor::ComponentVisitor;
use clap::Args;

use std::path::PathBuf;

use crate::components::component::walk_components;
use crate::constants::CRANE_FILE;
use crate::utils::parser;
use crate::visitors::sync_visitor::ComponentSyncVisitor;

#[derive(Args, Debug)]
pub struct CommandArgs {
    pub dir: Option<PathBuf>,
}

pub async fn run(args: &CommandArgs) -> Result<(), Error> {
    let visitor = ComponentSyncVisitor::new();
    if let Some(target_dir) = &args.dir {
        println!("Sync in directory {:?}", target_dir);
        walk_components(&visitor, target_dir, None).await
    } else {
        println!("Syncing current directory");
        walk_components(&visitor, &PathBuf::from("."), None).await
    }
}
