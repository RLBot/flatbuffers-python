use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum JumpMutator {
    #[default]
    Default = 0,
    Grounded = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Unlimited = 5,
    NoJumps = 6,
}

impl From<flat::JumpMutator> for JumpMutator {
    fn from(flat_t: flat::JumpMutator) -> Self {
        match flat_t {
            flat::JumpMutator::Default => Self::Default,
            flat::JumpMutator::Grounded => Self::Grounded,
            flat::JumpMutator::Two => Self::Two,
            flat::JumpMutator::Three => Self::Three,
            flat::JumpMutator::Four => Self::Four,
            flat::JumpMutator::Unlimited => Self::Unlimited,
            flat::JumpMutator::NoJumps => Self::NoJumps,
        }
    }
}

impl From<&JumpMutator> for flat::JumpMutator {
    fn from(py_type: &JumpMutator) -> Self {
        match py_type {
            JumpMutator::Default => Self::Default,
            JumpMutator::Grounded => Self::Grounded,
            JumpMutator::Two => Self::Two,
            JumpMutator::Three => Self::Three,
            JumpMutator::Four => Self::Four,
            JumpMutator::Unlimited => Self::Unlimited,
            JumpMutator::NoJumps => Self::NoJumps,
        }
    }
}

#[pymethods]
impl JumpMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Grounded),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Unlimited),
            6 => Ok(Self::NoJumps),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("JumpMutator.{self:?}")
    }
}
