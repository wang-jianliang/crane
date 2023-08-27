use std::path::PathBuf;

use crate::components::component::{walk_components, ComponentArena, ComponentID};
use crate::components::git_dependency::GitDependency;
use crate::errors::Error;
use crate::utils::git_utils::{
    add_alternate, checkout_to_target, fetch_repository, open_or_create_repo,
};
use crate::utils::{cache::ensure_cache_dir, encode::string_to_base64, parser};
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
        self.visit_git(id, root_dir).await?;

        // Handle deps if necessary
        let deps_file;
        let solution_name;
        let target_dir;
        {
            let comp = ComponentArena::instance().get(id).unwrap();
            let solution = match comp.impl_.as_any().downcast_ref::<GitDependency>() {
                Some(s) => s,
                None => {
                    return Err(Error {
                        message: format!(
                            "expect type of Solution, but got {:?}: {:?}",
                            comp.type_, comp.impl_
                        ),
                    })
                }
            };
            deps_file = solution.deps_file.clone();
            solution_name = comp.name.clone();
            target_dir = comp.target_dir.clone();
        }

        if let Some(deps_file) = &deps_file {
            let deps_file_path = root_dir.join(PathBuf::from(deps_file));
            let deps = parser::parse_components(&deps_file_path, "deps")?;
            walk_components(deps, ComponentSyncVisitor::new(), &target_dir).await?
        }
        println!("Solution {} is done", solution_name);
        Ok(())
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

        log::debug!("check to {}", head);
        checkout_to_target(&repo, &head)?;

        Ok(())
    }
}
