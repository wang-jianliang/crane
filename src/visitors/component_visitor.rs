use crate::components::component::ComponentID;
use crate::errors::Error;

pub trait ComponentVisitor: std::marker::Copy {
    fn visit_solution(&self, id: ComponentID) -> Result<(), Error>;
    fn visit_git(&self, id: ComponentID) -> Result<(), Error>;
}
