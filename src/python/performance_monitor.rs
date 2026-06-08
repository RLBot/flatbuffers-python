use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::PerformanceMonitor;

#[pymethods]
impl PerformanceMonitor {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::ShowWhenSuboptimal),
            1 => Ok(Self::AlwaysShow),
            2 => Ok(Self::NeverShow),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("PerformanceMonitor.{self:?}")
    }
}
