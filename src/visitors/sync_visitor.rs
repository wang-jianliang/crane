use std::path::PathBuf;

use crate::components::component::{ComponentArena, ComponentID};
use crate::components::git_dependency::GitDependency;
use crate::errors::Error;
use crate::utils::git_utils::{
    add_alternate, checkout_to_target, fetch_repository, open_or_create_repo,
};
use crate::utils::{cache::ensure_cache_dir, encode::string_to_base64};
use crate::visitors::component_visitor::ComponentVisitor;
use async_trait::async_trait;

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
        let name;
        let url;
        let target_dir;
        let refspec;
        let head;
        {
            let comp = ComponentArena::instance().get(id).unwrap();
            let git = comp.impl_.as_any().downcast_ref::<GitDependency>().unwrap();
            name = comp.name.clone();
            url = git.url.clone();
            target_dir = root_dir.clone().join(&comp.target_dir);

            if let Some(commit) = &git.commit {
                refspec = commit.clone();
                head = commit.clone();
            } else if let Some(branch) = &git.branch {
                refspec = format!("+refs/heads/{}:refs/heads/{}", branch, branch);
                head = branch.clone();
            } else {
                return Err(Error {
                    message: String::from("neither branch nor commit is specified"),
                });
            }
        }
        log::debug!("visit git component: {}", name);

        // Set up global cache
        log::debug!("set up git global cache for component {}", name);
        let cache_dir = ensure_cache_dir().join("git").join(string_to_base64(&url));
        log::debug!("create cache repository");
        let cache_repo = open_or_create_repo(&cache_dir)?;
        fetch_repository(&cache_repo, &url, &refspec)?;

        // The objects will be fetched from object database of cache repository
        let repo = open_or_create_repo(&target_dir)?;
        fetch_repository(&repo, &url, &refspec)?;

        add_alternate(&target_dir, &cache_dir.join(".git"))?;

        log::debug!("checkout to {}", head);
        checkout_to_target(&repo, &head)?;

        Ok(())
    }
}
