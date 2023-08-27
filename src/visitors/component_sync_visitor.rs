use std::path::PathBuf;

use crate::components::component::{walk_components, ComponentArena, ComponentID};
use crate::components::solution::Solution;
use crate::errors::Error;
use crate::utils::parser;
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
        let deps_file;
        let solution_name;
        let target_dir;
        {
            let comp = ComponentArena::instance().get(id).unwrap();
            let solution = comp.impl_.as_any().downcast_ref::<Solution>().unwrap();
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

    async fn visit_git(&self, id: ComponentID, _root_dir: &PathBuf) -> Result<(), Error> {
        let git = ComponentArena::instance().get(id).unwrap();
        log::debug!("visit git component: {:?}", git);
        Ok(())
    }
}
