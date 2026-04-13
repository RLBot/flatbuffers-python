use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum AudioMutator {
    #[default]
    Default = 0,
    Haunted = 1,
}

impl From<flat::AudioMutator> for AudioMutator {
    fn from(flat_t: flat::AudioMutator) -> Self {
        match flat_t {
            flat::AudioMutator::Default => Self::Default,
            flat::AudioMutator::Haunted => Self::Haunted,
        }
    }
}

impl From<&AudioMutator> for flat::AudioMutator {
    fn from(py_type: &AudioMutator) -> Self {
        match py_type {
            AudioMutator::Default => Self::Default,
            AudioMutator::Haunted => Self::Haunted,
        }
    }
}

#[pymethods]
impl AudioMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Haunted),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("AudioMutator.{self:?}")
    }
}
