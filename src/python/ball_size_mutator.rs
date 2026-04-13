use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BallSizeMutator {
    #[default]
    Default = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
    Gigantic = 4,
}

impl From<flat::BallSizeMutator> for BallSizeMutator {
    fn from(flat_t: flat::BallSizeMutator) -> Self {
        match flat_t {
            flat::BallSizeMutator::Default => Self::Default,
            flat::BallSizeMutator::Small => Self::Small,
            flat::BallSizeMutator::Medium => Self::Medium,
            flat::BallSizeMutator::Large => Self::Large,
            flat::BallSizeMutator::Gigantic => Self::Gigantic,
        }
    }
}

impl From<&BallSizeMutator> for flat::BallSizeMutator {
    fn from(py_type: &BallSizeMutator) -> Self {
        match py_type {
            BallSizeMutator::Default => Self::Default,
            BallSizeMutator::Small => Self::Small,
            BallSizeMutator::Medium => Self::Medium,
            BallSizeMutator::Large => Self::Large,
            BallSizeMutator::Gigantic => Self::Gigantic,
        }
    }
}

#[pymethods]
impl BallSizeMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Small),
            2 => Ok(Self::Medium),
            3 => Ok(Self::Large),
            4 => Ok(Self::Gigantic),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BallSizeMutator.{self:?}")
    }
}
