use std::path::PathBuf;

use crate::components::component::{ComponentArena, ComponentID};
use crate::errors::Error;
use crate::visitors::component_visitor::ComponentVisitor;
use async_trait::async_trait;

#[derive(Copy, Clone)]
pub struct StatusVisitor {}

impl StatusVisitor {
    pub fn new() -> Self {
        StatusVisitor {}
    }
}

#[async_trait]
impl ComponentVisitor for StatusVisitor {
    async fn visit_solution(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error> {
        self.visit_git(id, root_dir).await?;
        Ok(())
    }

    async fn visit_git(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error> {
        let name;
        let target_dir;
        {
            let comp = ComponentArena::instance().get(id).unwrap();
            name = comp.name.clone();
            target_dir = root_dir.clone().join(&comp.target_dir);
        }
        log::debug!("show status of git component: {} in {:?}", name, target_dir);
        Ok(())
    }
}
