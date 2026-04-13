use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum GameSpeedMutator {
    #[default]
    Default = 0,
    SloMo = 1,
    TimeWarp = 2,
}

impl From<flat::GameSpeedMutator> for GameSpeedMutator {
    fn from(flat_t: flat::GameSpeedMutator) -> Self {
        match flat_t {
            flat::GameSpeedMutator::Default => Self::Default,
            flat::GameSpeedMutator::SloMo => Self::SloMo,
            flat::GameSpeedMutator::TimeWarp => Self::TimeWarp,
        }
    }
}

impl From<&GameSpeedMutator> for flat::GameSpeedMutator {
    fn from(py_type: &GameSpeedMutator) -> Self {
        match py_type {
            GameSpeedMutator::Default => Self::Default,
            GameSpeedMutator::SloMo => Self::SloMo,
            GameSpeedMutator::TimeWarp => Self::TimeWarp,
        }
    }
}

#[pymethods]
impl GameSpeedMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::SloMo),
            2 => Ok(Self::TimeWarp),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("GameSpeedMutator.{self:?}")
    }
}
