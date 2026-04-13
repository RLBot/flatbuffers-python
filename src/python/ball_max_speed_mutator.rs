use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BallMaxSpeedMutator {
    #[default]
    Default = 0,
    Slow = 1,
    Fast = 2,
    SuperFast = 3,
}

impl From<flat::BallMaxSpeedMutator> for BallMaxSpeedMutator {
    fn from(flat_t: flat::BallMaxSpeedMutator) -> Self {
        match flat_t {
            flat::BallMaxSpeedMutator::Default => Self::Default,
            flat::BallMaxSpeedMutator::Slow => Self::Slow,
            flat::BallMaxSpeedMutator::Fast => Self::Fast,
            flat::BallMaxSpeedMutator::SuperFast => Self::SuperFast,
        }
    }
}

impl From<&BallMaxSpeedMutator> for flat::BallMaxSpeedMutator {
    fn from(py_type: &BallMaxSpeedMutator) -> Self {
        match py_type {
            BallMaxSpeedMutator::Default => Self::Default,
            BallMaxSpeedMutator::Slow => Self::Slow,
            BallMaxSpeedMutator::Fast => Self::Fast,
            BallMaxSpeedMutator::SuperFast => Self::SuperFast,
        }
    }
}

#[pymethods]
impl BallMaxSpeedMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Slow),
            2 => Ok(Self::Fast),
            3 => Ok(Self::SuperFast),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BallMaxSpeedMutator.{self:?}")
    }
}
