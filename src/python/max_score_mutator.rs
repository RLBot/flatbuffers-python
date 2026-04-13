use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::MaxScoreMutator;

#[pymethods]
impl MaxScoreMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Unlimited),
            1 => Ok(Self::OneGoal),
            2 => Ok(Self::ThreeGoals),
            3 => Ok(Self::FiveGoals),
            4 => Ok(Self::SevenGoals),
            5 => Ok(Self::TenGoals),
            6 => Ok(Self::TwentyGoals),
            7 => Ok(Self::ThirtyGoals),
            8 => Ok(Self::FortyGoals),
            9 => Ok(Self::FiftyGoals),
            10 => Ok(Self::SixtyGoals),
            11 => Ok(Self::SeventyGoals),
            12 => Ok(Self::EightyGoals),
            13 => Ok(Self::NinetyGoals),
            14 => Ok(Self::HundredGoals),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("MaxScoreMutator.{self:?}")
    }
}
