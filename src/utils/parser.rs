use rustpython_vm::builtins::PyDict;
use std::path::PathBuf;

use crate::components::component::{Component, ComponentID};
use crate::errors::Error;
use crate::utils::rustpython::format_py_exception;

// load the python format file .crane and parse the dict "solutions" in it
pub fn parse_components<'a>(
    config_file: &PathBuf,
    var_name: &str,
) -> Result<Vec<ComponentID>, Error> {
    log::debug!("parsing components defined in {:#?}", config_file);

    let interp = rustpython::InterpreterConfig::new()
        .init_stdlib()
        .interpreter();

    interp.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        vm.run_script(
            scope.clone(),
            config_file.clone().into_os_string().to_str().unwrap(),
        )
        .or_else(|err| Err(Error::new(format_py_exception(&err, vm))))?;

        let py_obj = match scope.globals.get_item(var_name, vm) {
            Ok(obj) => obj,
            Err(err) => {
                return Err(Error {
                    message: format!("Failed to get variable {}: {:?}", var_name, err),
                });
            }
        };

        let py_dict = py_obj.downcast::<PyDict>().unwrap();

        let mut components = vec![];

        for (key, value) in py_dict {
            let name: String = key.try_into_value(vm).unwrap();
            let comp = Component::from_py(name, &value, vm)?;
            components.push(comp);
        }

        log::debug!("Loaded components:\n{:#?}", components);
        Ok(components)
    })
}
