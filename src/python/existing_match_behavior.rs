use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::ExistingMatchBehavior;

#[pymethods]
impl ExistingMatchBehavior {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Restart),
            1 => Ok(Self::ContinueAndSpawn),
            2 => Ok(Self::RestartIfDifferent),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("ExistingMatchBehavior.{self:?}")
    }
}
