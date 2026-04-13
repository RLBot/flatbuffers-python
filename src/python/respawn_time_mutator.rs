use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum RespawnTimeMutator {
    #[default]
    ThreeSeconds = 0,
    TwoSeconds = 1,
    OneSecond = 2,
    DisableGoalReset = 3,
}

impl From<flat::RespawnTimeMutator> for RespawnTimeMutator {
    fn from(flat_t: flat::RespawnTimeMutator) -> Self {
        match flat_t {
            flat::RespawnTimeMutator::ThreeSeconds => Self::ThreeSeconds,
            flat::RespawnTimeMutator::TwoSeconds => Self::TwoSeconds,
            flat::RespawnTimeMutator::OneSecond => Self::OneSecond,
            flat::RespawnTimeMutator::DisableGoalReset => Self::DisableGoalReset,
        }
    }
}

impl From<&RespawnTimeMutator> for flat::RespawnTimeMutator {
    fn from(py_type: &RespawnTimeMutator) -> Self {
        match py_type {
            RespawnTimeMutator::ThreeSeconds => Self::ThreeSeconds,
            RespawnTimeMutator::TwoSeconds => Self::TwoSeconds,
            RespawnTimeMutator::OneSecond => Self::OneSecond,
            RespawnTimeMutator::DisableGoalReset => Self::DisableGoalReset,
        }
    }
}

#[pymethods]
impl RespawnTimeMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::ThreeSeconds),
            1 => Ok(Self::TwoSeconds),
            2 => Ok(Self::OneSecond),
            3 => Ok(Self::DisableGoalReset),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("RespawnTimeMutator.{self:?}")
    }
}
