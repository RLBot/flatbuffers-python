use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum MaxTimeMutator {
    #[default]
    Unlimited = 0,
    ElevenMinutes = 1,
}

impl From<flat::MaxTimeMutator> for MaxTimeMutator {
    fn from(flat_t: flat::MaxTimeMutator) -> Self {
        match flat_t {
            flat::MaxTimeMutator::Unlimited => Self::Unlimited,
            flat::MaxTimeMutator::ElevenMinutes => Self::ElevenMinutes,
        }
    }
}

impl From<&MaxTimeMutator> for flat::MaxTimeMutator {
    fn from(py_type: &MaxTimeMutator) -> Self {
        match py_type {
            MaxTimeMutator::Unlimited => Self::Unlimited,
            MaxTimeMutator::ElevenMinutes => Self::ElevenMinutes,
        }
    }
}

#[pymethods]
impl MaxTimeMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Unlimited),
            1 => Ok(Self::ElevenMinutes),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("MaxTimeMutator.{self:?}")
    }
}
