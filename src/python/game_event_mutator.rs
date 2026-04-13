use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum GameEventMutator {
    #[default]
    Default = 0,
    Haunted = 1,
    Rugby = 2,
}

impl From<flat::GameEventMutator> for GameEventMutator {
    fn from(flat_t: flat::GameEventMutator) -> Self {
        match flat_t {
            flat::GameEventMutator::Default => Self::Default,
            flat::GameEventMutator::Haunted => Self::Haunted,
            flat::GameEventMutator::Rugby => Self::Rugby,
        }
    }
}

impl From<&GameEventMutator> for flat::GameEventMutator {
    fn from(py_type: &GameEventMutator) -> Self {
        match py_type {
            GameEventMutator::Default => Self::Default,
            GameEventMutator::Haunted => Self::Haunted,
            GameEventMutator::Rugby => Self::Rugby,
        }
    }
}

#[pymethods]
impl GameEventMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Haunted),
            2 => Ok(Self::Rugby),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("GameEventMutator.{self:?}")
    }
}
