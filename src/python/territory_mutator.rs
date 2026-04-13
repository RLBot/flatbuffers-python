use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum TerritoryMutator {
    #[default]
    Off = 0,
    Territory = 1,
}

impl From<flat::TerritoryMutator> for TerritoryMutator {
    fn from(flat_t: flat::TerritoryMutator) -> Self {
        match flat_t {
            flat::TerritoryMutator::Off => Self::Off,
            flat::TerritoryMutator::Territory => Self::Territory,
        }
    }
}

impl From<&TerritoryMutator> for flat::TerritoryMutator {
    fn from(py_type: &TerritoryMutator) -> Self {
        match py_type {
            TerritoryMutator::Off => Self::Off,
            TerritoryMutator::Territory => Self::Territory,
        }
    }
}

#[pymethods]
impl TerritoryMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::Territory),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("TerritoryMutator.{self:?}")
    }
}
