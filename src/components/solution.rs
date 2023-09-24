use std::any::Any;

use crate::components::component::{ComponentImpl, FromPyObject};

use crane_derive::FromPyObject;
use rustpython_vm::{PyObjectRef, VirtualMachine};

use crate::errors::Error;

#[derive(Debug, FromPyObject)]
pub struct Solution {
    #[from_py]
    pub name: String,
    #[from_py]
    pub src: String,
    #[from_py]
    pub deps_file: Option<String>,
}

impl Default for Solution {
    fn default() -> Self {
        Solution {
            name: String::from(""),
            src: String::from(""),
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
