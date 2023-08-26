use std::any::Any;

use crate::components::component::{ComponentImpl, FromPyObject};

use crane_derive::FromPyObject;
use pyo3::prelude::*;


#[derive(Debug, FromPyObject)]
pub struct GitDependency {
    #[from_py]
    paths: Option<Vec<String>>,
}

impl Default for GitDependency {
    fn default() -> Self {
        GitDependency { paths: None }
    }
}

impl ComponentImpl for GitDependency {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

}
