use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::PathBuf;

use crate::components::component::{Component, FromPyObject};
use crate::components::git_dependency::GitDependency;
use crate::components::solution::Solution;
use crate::errors::Error;

// load the python format file .crane and parse the dict "solutions" in it
pub fn parse_components(
    config_file: &PathBuf,
    var_name: &str,
) -> Result<Vec<Box<dyn Component>>, Error> {
    pyo3::prepare_freethreaded_python();
    // evaluate the python file and return the dict "solutions"
    Python::with_gil(|py| {
        let module =
            match PyModule::from_code(py, &std::fs::read_to_string(config_file).unwrap(), "", "") {
                Ok(m) => m,
                Err(err) => {
                    return Err(Error {
                        message: format!("{}", err),
                    });
                }
            };

        let components: &PyList = module.getattr(var_name).unwrap().downcast().unwrap();

        let mut result = vec![];

        for component in components.iter() {
            let source_type = match var_name {
                "solutions" => String::from("solution"),
                _ => component
                    .get_item("type")
                    .unwrap()
                    .extract::<String>()
                    .unwrap(),
            };

            let c: Box<dyn Component> = match &source_type as &str {
                "solution" => Box::new(Solution::from_py(component)),
                "git" => Box::new(GitDependency::from_py(component)),
                unknown => {
                    log::warn!("Unsupported type {}", unknown);
                    continue;
                }
            };

            result.push(c);
        }

        log::debug!("Loaded components:\n{:#?}", result);
        Ok(result)
    })
}
