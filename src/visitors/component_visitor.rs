use crate::components::component::ComponentID;
use crate::errors::Error;
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait ComponentVisitor: std::marker::Copy {
    async fn visit_solution(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error>;
    async fn visit_git(&self, id: ComponentID, root_dir: &PathBuf) -> Result<(), Error>;
}
