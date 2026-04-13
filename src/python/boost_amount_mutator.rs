use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BoostAmountMutator {
    #[default]
    NormalBoost = 0,
    UnlimitedBoost = 1,
    SlowRecharge = 2,
    RapidRecharge = 3,
    NoBoost = 4,
}

impl From<flat::BoostAmountMutator> for BoostAmountMutator {
    fn from(flat_t: flat::BoostAmountMutator) -> Self {
        match flat_t {
            flat::BoostAmountMutator::NormalBoost => Self::NormalBoost,
            flat::BoostAmountMutator::UnlimitedBoost => Self::UnlimitedBoost,
            flat::BoostAmountMutator::SlowRecharge => Self::SlowRecharge,
            flat::BoostAmountMutator::RapidRecharge => Self::RapidRecharge,
            flat::BoostAmountMutator::NoBoost => Self::NoBoost,
        }
    }
}

impl From<&BoostAmountMutator> for flat::BoostAmountMutator {
    fn from(py_type: &BoostAmountMutator) -> Self {
        match py_type {
            BoostAmountMutator::NormalBoost => Self::NormalBoost,
            BoostAmountMutator::UnlimitedBoost => Self::UnlimitedBoost,
            BoostAmountMutator::SlowRecharge => Self::SlowRecharge,
            BoostAmountMutator::RapidRecharge => Self::RapidRecharge,
            BoostAmountMutator::NoBoost => Self::NoBoost,
        }
    }
}

#[pymethods]
impl BoostAmountMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::NormalBoost),
            1 => Ok(Self::UnlimitedBoost),
            2 => Ok(Self::SlowRecharge),
            3 => Ok(Self::RapidRecharge),
            4 => Ok(Self::NoBoost),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BoostAmountMutator.{self:?}")
    }
}
