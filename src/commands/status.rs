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

    let crane_file = root_dir.join(PathBuf::from(CRANE_FILE));
    if !crane_file.exists() {
        return Err(Error {
            message: String::from(format!(
                "Can not find a valid config file in path {:?}",
                crane_file
            )),
        });
    }
    let full_path: PathBuf = std::fs::canonicalize(&crane_file)
        .expect(format!("Failed to get absolute path of {:?}", crane_file).as_str());

    let deps = parser::parse_components(&full_path, "deps")?;

    walk_components(deps, &StatusVisitor::new(), &root_dir).await?;
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
