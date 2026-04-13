use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::DemolishMutator;

#[pymethods]
impl DemolishMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Disabled),
            2 => Ok(Self::FriendlyFire),
            3 => Ok(Self::OnContact),
            4 => Ok(Self::OnContactFf),
            5 => Ok(Self::OnBallContact),
            6 => Ok(Self::OnBallContactFf),
            7 => Ok(Self::OnBallContactSilent),
            8 => Ok(Self::OnBallContactFfSilent),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("DemolishMutator.{self:?}")
    }
}
