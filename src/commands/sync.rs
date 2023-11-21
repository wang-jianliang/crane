use crate::errors::Error;
use crate::utils::git_utils;
use clap::Args;
use git2::Repository;

use std::env;
use std::path::{Path, PathBuf};

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
    #[clap(long)]
    pub commit: Option<String>,
    #[clap(long)]
    pub remote: Option<String>,
}

async fn do_sync(
    url: Option<String>,
    branch: Option<String>,
    commit: Option<String>,
    root_dir: Option<PathBuf>,
    remote_name: &str,
) -> Result<(), Error> {
    let url_str;
    let abs_root_dir;
    let mut target_branch = branch.clone();
    let mut target_commit = commit.clone();

    log::debug!(
        "do sync: url: {:?} branch: {:?} commit: {:?} root_dir: {:?} remote_name: {:?}",
        url,
        branch,
        commit,
        root_dir,
        remote_name
    );

    // url: parse root_dir from url
    // root_dir: get url from root_dir
    // url + root_dir: return an error if root_dir exists
    match (&url, &root_dir) {
        (Some(u), Some(root_dir)) => {
            abs_root_dir = env::current_dir()?.join(root_dir);
            url_str = u.clone();
            // Check if root_dir exsits first
            if Path::new(&abs_root_dir).exists() {
                return Err(Error {
                    message: format!("The directory {} exists", root_dir.display()),
                });
            }
        }
        (Some(u), None) => {
            let repo_name = git_utils::get_repo_name(&u).ok_or(Error {
                message: format!("Failed to get repo name from url {}", u),
            })?;
            abs_root_dir = env::current_dir()?.join(repo_name);
            url_str = u.clone();
            target_branch = target_branch.or(git_utils::get_remote_default_branch(
                &url.unwrap(),
                Some(remote_name),
            ));
        }
        (None, _) => {
            match root_dir {
                Some(dir) => {
                    abs_root_dir = env::current_dir()?.join(dir);
                }
                None => {
                    abs_root_dir = env::current_dir()?;
                }
            }
            let repo = Repository::open(&abs_root_dir)?;
            url_str = repo
                .find_remote("origin")?
                .url()
                .ok_or(Error {
                    message: "Remote url is not set".to_string(),
                })?
                .to_string();

            // If the target_branch is None, we try to find it from the repository in root_dir
            target_branch = target_branch.or_else(|| {
                let repo = Repository::open(&abs_root_dir).ok()?;
                let head = repo.head().ok()?;
                let b = head.shorthand().map(|b| b.to_string());

                // If the target_branch can not be found, we try to set the head commit
                if b.is_none() {
                    target_commit = commit.or(head.target().map(|c| c.to_string()));
                }
                b
            });
        }
    }

    println!("Sync solution to {}", abs_root_dir.display());

    let visitor = ComponentSyncVisitor::new();
    let _ = visit_root_solution(
        &visitor,
        &abs_root_dir,
        url_str.to_string(),
        target_branch,
        target_commit,
        Some(CRANE_FILE.to_string()),
    )
    .await?;

    Ok(())
}

/* Possible usages:
 * 1. Sync an existing solution with the solution directory:
 *   crane sync
 * 2. Sync an existing solution in provided directory
 *   crane sync <dir>
 * 3. Sync a new solution with url
 *   crane sync --url https://xxx.git
 * 4. Sync a new solution to provided directory
 *   crane sync <dir> --url https://xxx.git
 * 5. Sync a new solution with url and branch
 *   crane sync --url https://xxx.git --branch main
*/
pub async fn run(args: &CommandArgs) -> Result<(), Error> {
    do_sync(
        args.url.clone(),
        args.branch.clone(),
        args.commit.clone(),
        args.dir.clone(),
        args.remote.clone().unwrap_or("origin".to_string()).as_str(),
    )
    .await
}
