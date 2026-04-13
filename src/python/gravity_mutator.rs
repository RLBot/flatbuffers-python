use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum GravityMutator {
    #[default]
    Default = 0,
    Low = 1,
    High = 2,
    SuperHigh = 3,
    Reverse = 4,
}

impl From<flat::GravityMutator> for GravityMutator {
    fn from(flat_t: flat::GravityMutator) -> Self {
        match flat_t {
            flat::GravityMutator::Default => Self::Default,
            flat::GravityMutator::Low => Self::Low,
            flat::GravityMutator::High => Self::High,
            flat::GravityMutator::SuperHigh => Self::SuperHigh,
            flat::GravityMutator::Reverse => Self::Reverse,
        }
    }
}

impl From<&GravityMutator> for flat::GravityMutator {
    fn from(py_type: &GravityMutator) -> Self {
        match py_type {
            GravityMutator::Default => Self::Default,
            GravityMutator::Low => Self::Low,
            GravityMutator::High => Self::High,
            GravityMutator::SuperHigh => Self::SuperHigh,
            GravityMutator::Reverse => Self::Reverse,
        }
    }
}

#[pymethods]
impl GravityMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Low),
            2 => Ok(Self::High),
            3 => Ok(Self::SuperHigh),
            4 => Ok(Self::Reverse),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("GravityMutator.{self:?}")
    }
}
