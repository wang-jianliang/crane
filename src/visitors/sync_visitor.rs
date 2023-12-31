use std::path::PathBuf;

use crate::components::component::{ComponentArena, ComponentID};
use crate::components::git_dependency::GitDependency;
use crate::errors::Error;
use crate::utils::git_utils::{self, add_alternate, fetch_repository, open_or_create_repo};
use crate::utils::{cache::ensure_cache_dir, encode::string_to_base64};
use crate::visitors::component_visitor::ComponentVisitor;
use async_trait::async_trait;
use git2::{AnnotatedCommit, Oid, Repository};

pub fn fetch_with_alternate<'a>(
    repo: &'a Repository,
    refs: &[&str],
    remote_url: &'a str,
    remote_name: Option<&str>,
) -> Result<AnnotatedCommit<'a>, Error> {
    // Set up global cache
    log::debug!(
        "set up git global cache for repository {}",
        repo.path().display()
    );
    let cache_dir = ensure_cache_dir()
        .join("git")
        .join(string_to_base64(&remote_url.to_string()));
    log::debug!("create cache repository");
    let cache_repo = open_or_create_repo(&cache_dir)?;
    fetch_repository(&cache_repo, &remote_url, refs, remote_name)?;

    // The objects will be fetched from object database of cache repository
    add_alternate(&repo.workdir().unwrap(), &cache_dir.join(".git"))?;
    let fetch_head = fetch_repository(&repo, &remote_url, refs, remote_name)?;
    Ok(fetch_head)
}

#[derive(Copy, Clone)]
pub struct ComponentSyncVisitor {}

impl ComponentSyncVisitor {
    pub fn new() -> Self {
        ComponentSyncVisitor {}
    }
}

#[async_trait]
impl ComponentVisitor for ComponentSyncVisitor {
    async fn visit_solution(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error> {
        // A solution should be a git repository
        self.visit_git(id, root_dir).await
    }

    async fn visit_git(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error> {
        let comp = ComponentArena::instance().get(id).unwrap();
        let git = comp.impl_.as_any().downcast_ref::<GitDependency>().unwrap();
        let name = comp.name.clone();
        let url = git.url.clone();
        let target_dir = root_dir.clone().join(&comp.target_dir);

        let repo = open_or_create_repo(&target_dir)?;
        let fetch_head;

        if let Some(branch) = &git.branch {
            let refname = format!("refs/for/{}", branch);
            fetch_head = fetch_with_alternate(&repo, &[branch], &url, Some("origin"))?;
            let mut msg = format!("Setting {} to {}", branch, fetch_head.id());

            let mut reference = match repo.find_reference(&refname) {
                Ok(r) => r,
                Err(_) => repo.reference(&refname, fetch_head.id(), true, &msg)?,
            };
            if let Some(commit) = &git.commit {
                // TODO: check if the commit exists on the branch
                msg = format!("Setting {} to certain commit {}", branch, commit);
                reference.set_target(Oid::from_str(&commit)?, &msg)?;
                git_utils::checkout_to_target(&repo, &commit)?;
            } else {
                reference.set_target(fetch_head.id(), &msg)?;
                git_utils::checkout_to_target(&repo, &fetch_head.id().to_string())?;
            }
            log::debug!("{}", msg);
            repo.set_head(&refname)?;
        } else if let Some(commit) = &git.commit {
            log::debug!("Set HEAD to {}", commit);
            fetch_with_alternate(&repo, &[&commit], &url, Some("origin"))?;
            git_utils::checkout_to_target(&repo, &commit)?;
            repo.set_head(&commit)?;
        } else {
            return Err(Error {
                message: String::from("neither branch nor commit is specified"),
            });
        }

        log::debug!("visit git component: {}", name);
        log::debug!(
            "checkout to {}",
            repo.head()?.target().map(|o| o.to_string()).unwrap()
        );

        Ok(())
    }
}
