use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BallBouncinessMutator {
    #[default]
    Default = 0,
    Low = 1,
    High = 2,
    SuperHigh = 3,
    Lowish = 4,
}

impl From<flat::BallBouncinessMutator> for BallBouncinessMutator {
    fn from(flat_t: flat::BallBouncinessMutator) -> Self {
        match flat_t {
            flat::BallBouncinessMutator::Default => Self::Default,
            flat::BallBouncinessMutator::Low => Self::Low,
            flat::BallBouncinessMutator::High => Self::High,
            flat::BallBouncinessMutator::SuperHigh => Self::SuperHigh,
            flat::BallBouncinessMutator::Lowish => Self::Lowish,
        }
    }
}

impl From<&BallBouncinessMutator> for flat::BallBouncinessMutator {
    fn from(py_type: &BallBouncinessMutator) -> Self {
        match py_type {
            BallBouncinessMutator::Default => Self::Default,
            BallBouncinessMutator::Low => Self::Low,
            BallBouncinessMutator::High => Self::High,
            BallBouncinessMutator::SuperHigh => Self::SuperHigh,
            BallBouncinessMutator::Lowish => Self::Lowish,
        }
    }
}

#[pymethods]
impl BallBouncinessMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Low),
            2 => Ok(Self::High),
            3 => Ok(Self::SuperHigh),
            4 => Ok(Self::Lowish),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BallBouncinessMutator.{self:?}")
    }
}
