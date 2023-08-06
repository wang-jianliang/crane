use crate::components::component::{Component, ComponentImpl, FromPyObject};
use crate::errors::Error;
use crane_derive::FromPyObject;
// use futures::future::try_join_all;
use pyo3::prelude::*;
use std::path::PathBuf;

use crate::utils::parser;

#[derive(Debug, FromPyObject)]
pub struct Solution {
    #[from_py]
    name: String,
    #[from_py]
    src: String,
    #[from_py]
    path: Option<PathBuf>,
    #[from_py]
    deps_file: Option<PathBuf>,
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
    fn sync(&self, comp: &Component) -> Result<(), Error> {
        if let Some(deps_file) = &self.deps_file {
            let deps = parser::parse_components(&deps_file, "deps")?;
        }
        println!("Solution {} is done", self.name);
        Ok(())
    }
}
