use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::BoostStrengthMutator;

#[pymethods]
impl BoostStrengthMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::OneAndAHalf),
            2 => Ok(Self::Two),
            3 => Ok(Self::Five),
            4 => Ok(Self::Ten),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BoostStrengthMutator.{self:?}")
    }
}
