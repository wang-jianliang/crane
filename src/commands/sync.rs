use crate::errors::Error;
use crate::utils::git_utils;
use clap::Args;
use git2::Repository;

use std::env;
use std::path::PathBuf;

use crate::components::component::visit_root_solution;
use crate::constants::CRANE_FILE;
use crate::visitors::sync_visitor::ComponentSyncVisitor;

#[derive(Args, Debug)]
pub struct CommandArgs {
    pub dir: Option<PathBuf>,
    #[clap(long)]
    pub url: Option<String>,
    #[clap(long)]
    pub branch: Option<String>,
}

/* Possible usages:
 * 1. Sync an existing solution with the solution directory:
 *   crane sync
 * 2. Sync an existing solution in provided directory
 *   crane sync <dir>
 * 3. Sync an new solution with url
 *   crane sync --url https://xxx.git
 * 4. Sync an new solution to provided directory
 *   crane sync <dir> --url https://xxx.git
*/
pub async fn run(args: &CommandArgs) -> Result<(), Error> {
    let visitor = ComponentSyncVisitor::new();

    let root_dir;
    let url;
    let branch;
    let commit;

    if let Some(u) = &args.url {
        let repo_name = git_utils::get_repo_name(&u).ok_or(Error {
            message: format!("Failed to get repo name from url {}", u),
        })?;
        root_dir = args.dir.clone().unwrap_or(PathBuf::from(repo_name));
        url = u.to_string();
        branch = args.branch.clone();
        commit = None;
    } else {
        root_dir = args.dir.clone().unwrap_or(PathBuf::from("."));
        let abs_root_dir = std::fs::canonicalize(root_dir.clone())
            .unwrap_or_else(|_| panic!("Failed to get absolute path of {:?}", root_dir));

        let repo = Repository::open(abs_root_dir)?;
        url = repo
            .find_remote("origin")?
            .url()
            .ok_or(Error {
                message: "Remote url is not set".to_string(),
            })?
            .to_string();
        let head = repo.head()?;
        branch = head.shorthand().map(|b| b.to_string());
        commit = if branch.is_none() {
            head.target().map(|c| c.to_string())
        } else {
            None
        }
    }
    let abs_root_dir = env::current_dir()?.join(root_dir);

    println!("Sync solution to {}", abs_root_dir.display());
    let _ = visit_root_solution(
        &visitor,
        &abs_root_dir,
        url,
        branch,
        commit,
        Some(CRANE_FILE.to_string()),
    )
    .await?;
    Ok(())
}
