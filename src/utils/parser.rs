use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::PathBuf;

use crate::components::component::Component;

// load the python format file .crane and parse the dict "solutions" in it
pub fn parse_components<T: Component + std::fmt::Debug>(config_file: &PathBuf, var_name: &str) -> Vec<T> {
    pyo3::prepare_freethreaded_python();
    // evaluate the python file and return the dict "solutions"
    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, &std::fs::read_to_string(config_file).unwrap(), "", "")
                .unwrap();

        let components: &PyList = module.getattr(var_name).unwrap().downcast().unwrap();

        let mut result = vec![];

        for component in components.iter() {
            let name = component
                .get_item("name")
                .unwrap()
                .extract::<String>()
                .unwrap();
            let path = component
                .get_item("path")
                .unwrap()
                .extract::<String>()
                .unwrap();
            let source_stamp = component
                .get_item("src")
                .unwrap()
                .extract::<String>()
                .unwrap();

            let s = T::new(&name, &path, &source_stamp);
            result.push(s);
        }

        log::debug!("Loaded components:\n{:#?}", result);
        result
    })
}
