use std::any::Any;

use crate::components::component::{Component, ComponentImpl, FromPyObject};
use crate::errors::Error;
use crane_derive::FromPyObject;
use pyo3::prelude::*;
use pyo3::types::PyDict;

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
