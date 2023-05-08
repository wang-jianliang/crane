use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::components::component::Component;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// load the python format file .crane and parse the dict "solutions" in it
pub fn parse_components<T: Component + std::fmt::Debug>(
    config_file: &PathBuf,
    var_name: &str,
) -> Vec<T> {
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
            component.del_item("name").unwrap();
            let path = component
                .get_item("path")
                .unwrap()
                .extract::<String>()
                .unwrap();
            component.del_item("path").unwrap();
            let source_stamp = component
                .get_item("src")
                .unwrap()
                .extract::<String>()
                .unwrap();
            component.del_item("src").unwrap();

            // collect other attributes
            let mut attrs = HashMap::new();
            let dict: &PyDict = component.downcast().unwrap();
            for (k, v) in dict {
                let key = k.extract::<String>().unwrap();
                let value = v.extract::<String>().unwrap();
                attrs.insert(key, value);
            }
            print_type_of(component);

            let s = T::new(&name, &path, &source_stamp, attrs);
            result.push(s);
        }

        log::debug!("Loaded components:\n{:#?}", result);
        result
    })
}
