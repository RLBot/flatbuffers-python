use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::PsyonixSkill;

#[pymethods]
impl PsyonixSkill {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Beginner),
            1 => Ok(Self::Rookie),
            2 => Ok(Self::Pro),
            3 => Ok(Self::AllStar),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("PsyonixSkill.{self:?}")
    }
}
