use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::AirState;

#[pymethods]
impl AirState {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::OnGround),
            1 => Ok(Self::Jumping),
            2 => Ok(Self::DoubleJumping),
            3 => Ok(Self::Dodging),
            4 => Ok(Self::InAir),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("AirState.{self:?}")
    }
}
