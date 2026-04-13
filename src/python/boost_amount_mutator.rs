use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::BoostAmountMutator;

#[pymethods]
impl BoostAmountMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::NormalBoost),
            1 => Ok(Self::UnlimitedBoost),
            2 => Ok(Self::SlowRecharge),
            3 => Ok(Self::RapidRecharge),
            4 => Ok(Self::NoBoost),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BoostAmountMutator.{self:?}")
    }
}
