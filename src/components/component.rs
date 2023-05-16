use pyo3::prelude::*;

pub trait AttrParser {
    fn from_py(py_obj: &PyAny) -> Self;
}

pub trait Component: std::fmt::Debug {
    fn sync(&self);
}
