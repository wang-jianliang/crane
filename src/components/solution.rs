use std::any::Any;

use crate::components::component::{Component, ComponentImpl, FromPyObject};
use crate::errors::Error;
use crane_derive::FromPyObject;
// use futures::future::try_join_all;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::PathBuf;

use crate::utils::parser;

#[derive(Debug, FromPyObject)]
pub struct Solution {
    #[from_py]
    pub name: String,
    #[from_py]
    pub src: String,
    #[from_py]
    pub path: Option<PathBuf>,
    #[from_py]
    pub deps_file: Option<PathBuf>,
}

impl Default for Solution {
    fn default() -> Self {
        Solution {
            name: String::from(""),
            src: String::from(""),
            path: None,
            deps_file: None,
        }
    }
}

impl ComponentImpl for Solution {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
