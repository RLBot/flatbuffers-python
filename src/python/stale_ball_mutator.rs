use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum StaleBallMutator {
    #[default]
    Unlimited = 0,
    ThirtySeconds = 1,
}

impl From<flat::StaleBallMutator> for StaleBallMutator {
    fn from(flat_t: flat::StaleBallMutator) -> Self {
        match flat_t {
            flat::StaleBallMutator::Unlimited => Self::Unlimited,
            flat::StaleBallMutator::ThirtySeconds => Self::ThirtySeconds,
        }
    }
}

impl From<&StaleBallMutator> for flat::StaleBallMutator {
    fn from(py_type: &StaleBallMutator) -> Self {
        match py_type {
            StaleBallMutator::Unlimited => Self::Unlimited,
            StaleBallMutator::ThirtySeconds => Self::ThirtySeconds,
        }
    }
}

#[pymethods]
impl StaleBallMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Unlimited),
            1 => Ok(Self::ThirtySeconds),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("StaleBallMutator.{self:?}")
    }
}
