use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum ExistingMatchBehavior {
    #[default]
    Restart = 0,
    ContinueAndSpawn = 1,
    RestartIfDifferent = 2,
}

impl From<flat::ExistingMatchBehavior> for ExistingMatchBehavior {
    fn from(flat_t: flat::ExistingMatchBehavior) -> Self {
        match flat_t {
            flat::ExistingMatchBehavior::Restart => Self::Restart,
            flat::ExistingMatchBehavior::ContinueAndSpawn => Self::ContinueAndSpawn,
            flat::ExistingMatchBehavior::RestartIfDifferent => Self::RestartIfDifferent,
        }
    }
}

impl From<&ExistingMatchBehavior> for flat::ExistingMatchBehavior {
    fn from(py_type: &ExistingMatchBehavior) -> Self {
        match py_type {
            ExistingMatchBehavior::Restart => Self::Restart,
            ExistingMatchBehavior::ContinueAndSpawn => Self::ContinueAndSpawn,
            ExistingMatchBehavior::RestartIfDifferent => Self::RestartIfDifferent,
        }
    }
}

#[pymethods]
impl ExistingMatchBehavior {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Restart),
            1 => Ok(Self::ContinueAndSpawn),
            2 => Ok(Self::RestartIfDifferent),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("ExistingMatchBehavior.{self:?}")
    }
}
