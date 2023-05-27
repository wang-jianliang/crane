use async_trait::async_trait;
use pyo3::prelude::*;

use crate::errors::Error;

pub trait FromPyObject {
    fn from_py(py_obj: &PyAny) -> Result<Self, PyErr>
    where
        Self: Sized;
}

#[async_trait]
pub trait Component: std::fmt::Debug + Send {
    async fn sync(&self) -> Result<(), Error>;
}
