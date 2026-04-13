use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum ScoringRuleMutator {
    #[default]
    Default = 0,
    Disabled = 1,
}

impl From<flat::ScoringRuleMutator> for ScoringRuleMutator {
    fn from(flat_t: flat::ScoringRuleMutator) -> Self {
        match flat_t {
            flat::ScoringRuleMutator::Default => Self::Default,
            flat::ScoringRuleMutator::Disabled => Self::Disabled,
        }
    }
}

impl From<&ScoringRuleMutator> for flat::ScoringRuleMutator {
    fn from(py_type: &ScoringRuleMutator) -> Self {
        match py_type {
            ScoringRuleMutator::Default => Self::Default,
            ScoringRuleMutator::Disabled => Self::Disabled,
        }
    }
}

#[pymethods]
impl ScoringRuleMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Disabled),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("ScoringRuleMutator.{self:?}")
    }
}
