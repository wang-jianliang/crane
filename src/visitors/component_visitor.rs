use crate::components::component::ComponentID;
use crate::components::{
    component::{walk_components, ComponentArena},
    git_dependency::GitDependency,
};
use crate::errors::Error;
use crate::utils::parser;
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait ComponentVisitor: std::marker::Copy + std::marker::Sync {
    async fn visit_solution(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error>;
    async fn visit_git(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error>;

    async fn visit_solution_with_deps(
        &self,
        id: ComponentID,
        root_dir: &PathBuf,
    ) -> Result<(), Error> {
        // A solution should be a git repository
        self.visit_git(id, root_dir).await?;

        // Handle deps if necessary
        let deps_file;
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
        }

        if let Some(deps_file) = &deps_file {
            let deps_file_path = root_dir.join(PathBuf::from(deps_file));
            walk_components(self, root_dir, Some(&deps_file_path)).await?
        }
        Ok(())
    }
}
