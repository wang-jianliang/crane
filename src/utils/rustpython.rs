use rustpython_vm::{builtins::PyBaseException, PyRef, VirtualMachine};

pub fn format_py_exception(exc: &PyRef<PyBaseException>, vm: &VirtualMachine) -> String {
    let mut msg = String::new();
    if let Err(err) = vm.write_exception(&mut msg, exc) {
        return format!("Failed to write exception message, err: {err}");
    }
    return msg;
}
