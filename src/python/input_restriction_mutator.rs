use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum InputRestrictionMutator {
    #[default]
    Default = 0,
    Backwards = 1,
}

impl From<flat::InputRestrictionMutator> for InputRestrictionMutator {
    fn from(flat_t: flat::InputRestrictionMutator) -> Self {
        match flat_t {
            flat::InputRestrictionMutator::Default => Self::Default,
            flat::InputRestrictionMutator::Backwards => Self::Backwards,
        }
    }
}

impl From<&InputRestrictionMutator> for flat::InputRestrictionMutator {
    fn from(py_type: &InputRestrictionMutator) -> Self {
        match py_type {
            InputRestrictionMutator::Default => Self::Default,
            InputRestrictionMutator::Backwards => Self::Backwards,
        }
    }
}

#[pymethods]
impl InputRestrictionMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Backwards),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("InputRestrictionMutator.{self:?}")
    }
}
