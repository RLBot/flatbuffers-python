use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BallGravityMutator {
    #[default]
    Default = 0,
    Low = 1,
    High = 2,
    SuperHigh = 3,
}

impl From<flat::BallGravityMutator> for BallGravityMutator {
    fn from(flat_t: flat::BallGravityMutator) -> Self {
        match flat_t {
            flat::BallGravityMutator::Default => Self::Default,
            flat::BallGravityMutator::Low => Self::Low,
            flat::BallGravityMutator::High => Self::High,
            flat::BallGravityMutator::SuperHigh => Self::SuperHigh,
        }
    }
}

impl From<&BallGravityMutator> for flat::BallGravityMutator {
    fn from(py_type: &BallGravityMutator) -> Self {
        match py_type {
            BallGravityMutator::Default => Self::Default,
            BallGravityMutator::Low => Self::Low,
            BallGravityMutator::High => Self::High,
            BallGravityMutator::SuperHigh => Self::SuperHigh,
        }
    }
}

#[pymethods]
impl BallGravityMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Low),
            2 => Ok(Self::High),
            3 => Ok(Self::SuperHigh),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BallGravityMutator.{self:?}")
    }
}
