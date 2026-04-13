use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::BallWeightMutator;

#[pymethods]
impl BallWeightMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Light),
            2 => Ok(Self::Heavy),
            3 => Ok(Self::SuperLight),
            4 => Ok(Self::CurveBall),
            5 => Ok(Self::BeachBallCurve),
            6 => Ok(Self::MagnusFutBall),
            7 => Ok(Self::MagnusFutballLess),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BallWeightMutator.{self:?}")
    }
}
