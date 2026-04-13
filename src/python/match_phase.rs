use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::MatchPhase;

#[pymethods]
impl MatchPhase {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Inactive),
            1 => Ok(Self::Countdown),
            2 => Ok(Self::Kickoff),
            3 => Ok(Self::Active),
            4 => Ok(Self::GoalScored),
            5 => Ok(Self::Replay),
            6 => Ok(Self::Paused),
            7 => Ok(Self::Ended),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("MatchPhase.{self:?}")
    }
}
