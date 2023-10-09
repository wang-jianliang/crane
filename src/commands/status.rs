use crate::errors::Error;
use clap::Args;

use std::path::PathBuf;

use crate::components::component::walk_components;
use crate::constants::CRANE_FILE;
use crate::utils::parser;
use crate::visitors::status_visitor::StatusVisitor;

#[derive(Args, Debug)]
pub struct CommandArgs {
    pub dir: Option<PathBuf>,
}

async fn show_status(root_dir: &PathBuf) -> Result<(), Error> {
    log::debug!("Show status in {:?}", root_dir);

    walk_components(&StatusVisitor::new(), &root_dir, None).await?;
    Ok(())
}

pub async fn run(args: &CommandArgs) -> Result<(), Error> {
    println!("{:?}", args);

    if let Some(target_dir) = &args.dir {
        show_status(target_dir).await
    } else {
        println!("Syncing current directory");
        show_status(&PathBuf::from(".")).await
    }
}
