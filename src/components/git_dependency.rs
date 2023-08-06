use crate::components::component::{Component, ComponentImpl, FromPyObject};
use crate::errors::Error;
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
    fn sync(&self, comp: &Component) -> Result<(), Error> {
        println!("GitDependency {} is done", comp.name);
        Ok(())
    }
}

