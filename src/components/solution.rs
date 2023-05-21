use crate::components::component::{Component, FromPyObject};
use crate::errors::Error;
use async_trait::async_trait;
use crane_derive::FromPyObject;
use pyo3::prelude::*;
use std::path::PathBuf;

use crate::utils::parser;

#[derive(Debug, FromPyObject)]
pub struct Solution {
    deps_file: Option<PathBuf>,
}

#[async_trait]
impl Component for Solution {
    async fn sync(&self) -> Result<(), Error> {
        if let Some(deps_file) = &self.deps_file {
            let deps: Vec<Box<dyn Component>> = parser::parse_components(&deps_file, "deps");
        }
        println!("sync");
        Ok(())
    }
}
