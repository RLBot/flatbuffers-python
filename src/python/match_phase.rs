use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum MatchPhase {
    #[default]
    Inactive = 0,
    Countdown = 1,
    Kickoff = 2,
    Active = 3,
    GoalScored = 4,
    Replay = 5,
    Paused = 6,
    Ended = 7,
}

impl From<flat::MatchPhase> for MatchPhase {
    fn from(flat_t: flat::MatchPhase) -> Self {
        match flat_t {
            flat::MatchPhase::Inactive => Self::Inactive,
            flat::MatchPhase::Countdown => Self::Countdown,
            flat::MatchPhase::Kickoff => Self::Kickoff,
            flat::MatchPhase::Active => Self::Active,
            flat::MatchPhase::GoalScored => Self::GoalScored,
            flat::MatchPhase::Replay => Self::Replay,
            flat::MatchPhase::Paused => Self::Paused,
            flat::MatchPhase::Ended => Self::Ended,
        }
    }
}

impl From<&MatchPhase> for flat::MatchPhase {
    fn from(py_type: &MatchPhase) -> Self {
        match py_type {
            MatchPhase::Inactive => Self::Inactive,
            MatchPhase::Countdown => Self::Countdown,
            MatchPhase::Kickoff => Self::Kickoff,
            MatchPhase::Active => Self::Active,
            MatchPhase::GoalScored => Self::GoalScored,
            MatchPhase::Replay => Self::Replay,
            MatchPhase::Paused => Self::Paused,
            MatchPhase::Ended => Self::Ended,
        }
    }
}

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
