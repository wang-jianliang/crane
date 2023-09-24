use std::any::Any;

use crate::components::component::{ComponentImpl, FromPyObject};

use crate::errors::Error;
use crane_derive::FromPyObject;
use rustpython_vm::{PyObjectRef, VirtualMachine};

#[derive(Debug, FromPyObject)]
pub struct GitDependency {
    #[from_py]
    paths: Option<Vec<String>>,
    #[from_py]
    pub url: String,
    #[from_py]
    pub commit: Option<String>,
    #[from_py]
    pub branch: Option<String>,
    #[from_py]
    pub deps_file: Option<String>,
}

impl Default for GitDependency {
    fn default() -> Self {
        GitDependency {
            paths: None,
            url: String::from(""),
            commit: None,
            branch: None,
            deps_file: None,
        }
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
