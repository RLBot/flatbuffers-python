use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum DodgeTimerMutator {
    #[default]
    OnePointTwentyFiveSeconds = 0,
    TwoSeconds = 1,
    ThreeSeconds = 2,
    Unlimited = 3,
}

impl From<flat::DodgeTimerMutator> for DodgeTimerMutator {
    fn from(flat_t: flat::DodgeTimerMutator) -> Self {
        match flat_t {
            flat::DodgeTimerMutator::OnePointTwentyFiveSeconds => Self::OnePointTwentyFiveSeconds,
            flat::DodgeTimerMutator::TwoSeconds => Self::TwoSeconds,
            flat::DodgeTimerMutator::ThreeSeconds => Self::ThreeSeconds,
            flat::DodgeTimerMutator::Unlimited => Self::Unlimited,
        }
    }
}

impl From<&DodgeTimerMutator> for flat::DodgeTimerMutator {
    fn from(py_type: &DodgeTimerMutator) -> Self {
        match py_type {
            DodgeTimerMutator::OnePointTwentyFiveSeconds => Self::OnePointTwentyFiveSeconds,
            DodgeTimerMutator::TwoSeconds => Self::TwoSeconds,
            DodgeTimerMutator::ThreeSeconds => Self::ThreeSeconds,
            DodgeTimerMutator::Unlimited => Self::Unlimited,
        }
    }
}

#[pymethods]
impl DodgeTimerMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::OnePointTwentyFiveSeconds),
            1 => Ok(Self::TwoSeconds),
            2 => Ok(Self::ThreeSeconds),
            3 => Ok(Self::Unlimited),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("DodgeTimerMutator.{self:?}")
    }
}
