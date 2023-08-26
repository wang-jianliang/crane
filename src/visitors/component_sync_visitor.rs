use crate::components::component::{ComponentArena, ComponentID};
use crate::components::solution::Solution;
use crate::errors::Error;
use crate::utils::parser;
use crate::visitors::component_visitor::ComponentVisitor;

#[derive(Copy, Clone)]
pub struct ComponentSyncVisitor {}

impl ComponentSyncVisitor {
    pub fn new() -> Self {
        ComponentSyncVisitor {}
    }
}

impl ComponentVisitor for ComponentSyncVisitor {
    fn visit_solution(&self, id: ComponentID) -> Result<(), Error> {
        let deps_file;
        let solution_name;
        {
            let comp = ComponentArena::instance().get(id).unwrap();
            let solution = comp.impl_.as_any().downcast_ref::<Solution>().unwrap();
            deps_file = solution.deps_file.clone();
            solution_name = solution.name.clone();
        }

        if let Some(deps_file) = &deps_file {
            let _deps = parser::parse_components(&deps_file, "deps")?;
            println!("Solution {} is done", solution_name);
        }
        Ok(())
    }

    fn visit_git(&self, id: ComponentID) -> Result<(), Error> {
        let git = ComponentArena::instance().get(id).unwrap();
        println!("{:?}", git);
        Ok(())
    }
}
