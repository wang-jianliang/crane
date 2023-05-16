use crate::components::component::{AttrParser, Component};
use crane_derive::AttrParser;
use pyo3::prelude::*;

#[derive(Debug, AttrParser)]
pub struct GitDependency {
    paths: Option<Vec<String>>,
}

impl Component for GitDependency {
    fn sync(&self) {
        println!("sync git dependency");
    }
}
