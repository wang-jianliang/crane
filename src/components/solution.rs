use crate::components::component::{Component, FromPyObject};
use crate::errors::Error;
use async_trait::async_trait;
use crane_derive::FromPyObject;
use futures::future::try_join_all;
use pyo3::prelude::*;
use std::path::PathBuf;

use crate::utils::parser;

#[derive(Debug, FromPyObject)]
pub struct Solution {
    name: String,
    src: String,
    path: Option<PathBuf>,
    deps_file: Option<PathBuf>,
}

#[async_trait]
impl Component for Solution {
    async fn sync(&self) -> Result<(), Error> {
        if let Some(deps_file) = &self.deps_file {
            let deps = parser::parse_components(&deps_file, "deps")?;
            let mut futures = Vec::new();
            // let futures = deps.into_iter().map(|d| d.sync());
            for dep in &deps {
                futures.push(dep.sync());
            }
            let _ = try_join_all(futures).await;
        }
        println!("Solution {} is done", self.name);
        Ok(())
    }
}
