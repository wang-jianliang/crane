use crate::components::component::{AttrParser, Component};
use crate::errors::Error;
use async_trait::async_trait;
use crane_derive::AttrParser;
use pyo3::prelude::*;

#[derive(Debug, AttrParser)]
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
