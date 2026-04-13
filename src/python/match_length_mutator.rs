use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum MatchLengthMutator {
    #[default]
    FiveMinutes = 0,
    TenMinutes = 1,
    TwentyMinutes = 2,
    Unlimited = 3,
}

impl From<flat::MatchLengthMutator> for MatchLengthMutator {
    fn from(flat_t: flat::MatchLengthMutator) -> Self {
        match flat_t {
            flat::MatchLengthMutator::FiveMinutes => Self::FiveMinutes,
            flat::MatchLengthMutator::TenMinutes => Self::TenMinutes,
            flat::MatchLengthMutator::TwentyMinutes => Self::TwentyMinutes,
            flat::MatchLengthMutator::Unlimited => Self::Unlimited,
        }
    }
}

impl From<&MatchLengthMutator> for flat::MatchLengthMutator {
    fn from(py_type: &MatchLengthMutator) -> Self {
        match py_type {
            MatchLengthMutator::FiveMinutes => Self::FiveMinutes,
            MatchLengthMutator::TenMinutes => Self::TenMinutes,
            MatchLengthMutator::TwentyMinutes => Self::TwentyMinutes,
            MatchLengthMutator::Unlimited => Self::Unlimited,
        }
    }
}

#[pymethods]
impl MatchLengthMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::FiveMinutes),
            1 => Ok(Self::TenMinutes),
            2 => Ok(Self::TwentyMinutes),
            3 => Ok(Self::Unlimited),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("MatchLengthMutator.{self:?}")
    }
}
