use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum OvertimeMutator {
    #[default]
    Unlimited = 0,
    FiveMaxFirstScore = 1,
    FiveMaxRandomTeam = 2,
}

impl From<flat::OvertimeMutator> for OvertimeMutator {
    fn from(flat_t: flat::OvertimeMutator) -> Self {
        match flat_t {
            flat::OvertimeMutator::Unlimited => Self::Unlimited,
            flat::OvertimeMutator::FiveMaxFirstScore => Self::FiveMaxFirstScore,
            flat::OvertimeMutator::FiveMaxRandomTeam => Self::FiveMaxRandomTeam,
        }
    }
}

impl From<&OvertimeMutator> for flat::OvertimeMutator {
    fn from(py_type: &OvertimeMutator) -> Self {
        match py_type {
            OvertimeMutator::Unlimited => Self::Unlimited,
            OvertimeMutator::FiveMaxFirstScore => Self::FiveMaxFirstScore,
            OvertimeMutator::FiveMaxRandomTeam => Self::FiveMaxRandomTeam,
        }
    }
}

#[pymethods]
impl OvertimeMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Unlimited),
            1 => Ok(Self::FiveMaxFirstScore),
            2 => Ok(Self::FiveMaxRandomTeam),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("OvertimeMutator.{self:?}")
    }
}
