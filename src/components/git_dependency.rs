use crate::components::component::{Component, FromPyObject};
use crate::errors::Error;
use async_trait::async_trait;
use crane_derive::FromPyObject;
use pyo3::prelude::*;

#[derive(Debug, FromPyObject)]
pub struct GitDependency {
    paths: Option<Vec<String>>,
}

#[async_trait]
impl Component for GitDependency {
    async fn sync(&self) -> Result<(), Error> {
        println!("sync git dependency");
        Ok(())
    }
}