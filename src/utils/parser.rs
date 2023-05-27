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

        let py_components: &PyList = module.getattr(var_name).unwrap().downcast().unwrap();

        let mut components = vec![];

        for component in py_components.iter() {
            let source_type = match var_name {
                "solutions" => String::from("solution"),
                _ => component
                    .get_item("type")
                    .unwrap()
                    .extract::<String>()
                    .unwrap(),
            };

            let result: Box<dyn Component> = match source_type.as_str() {
                "solution" => {
                    let solution = Solution::from_py(component)?.map_err(|err| Error {
                        message: format!("Failed to parse solution: {}", err),
                    })?;
                    Box::new(solution)
                }
                "git" => {
                    let git = GitDependency::from_py(component).map_err(|err| Error {
                        message: format!("Failed to parse git dependency: {}", err),
                    })?;
                    Box::new(git)
                }
                unknown => {
                    log::warn!("Unsupported type {}", unknown);
                    continue;
                }
            };

            components.push(result);
        }

        log::debug!("Loaded components:\n{:#?}", components);
        Ok(components)
    })
}
