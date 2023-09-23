use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::PathBuf;

use crate::components::component::{Component, ComponentID};
use crate::errors::Error;

// load the python format file .crane and parse the dict "solutions" in it
pub fn parse_components<'a>(
    config_file: &PathBuf,
    var_name: &str,
) -> Result<Vec<ComponentID>, Error> {
    log::debug!("parsing components defined in {:#?}", config_file);
    pyo3::prepare_freethreaded_python();
    // evaluate the python file and return the dict
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

        let py_objs: &PyDict = module.getattr(var_name).unwrap().downcast().unwrap();

        let mut components = vec![];

        for (key, obj) in py_objs.iter() {
            let name: String = key.to_string();
            let comp = Component::from_py(name, obj)?;
            components.push(comp);
        }
        log::debug!("Loaded components:\n{:#?}", components);
        Ok(components)
    })
}
