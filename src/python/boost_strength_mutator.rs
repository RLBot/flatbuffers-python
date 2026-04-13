use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BoostStrengthMutator {
    #[default]
    One = 0,
    OneAndAHalf = 1,
    Two = 2,
    Five = 3,
    Ten = 4,
}

impl From<flat::BoostStrengthMutator> for BoostStrengthMutator {
    fn from(flat_t: flat::BoostStrengthMutator) -> Self {
        match flat_t {
            flat::BoostStrengthMutator::One => Self::One,
            flat::BoostStrengthMutator::OneAndAHalf => Self::OneAndAHalf,
            flat::BoostStrengthMutator::Two => Self::Two,
            flat::BoostStrengthMutator::Five => Self::Five,
            flat::BoostStrengthMutator::Ten => Self::Ten,
        }
    }
}

impl From<&BoostStrengthMutator> for flat::BoostStrengthMutator {
    fn from(py_type: &BoostStrengthMutator) -> Self {
        match py_type {
            BoostStrengthMutator::One => Self::One,
            BoostStrengthMutator::OneAndAHalf => Self::OneAndAHalf,
            BoostStrengthMutator::Two => Self::Two,
            BoostStrengthMutator::Five => Self::Five,
            BoostStrengthMutator::Ten => Self::Ten,
        }
    }
}

#[pymethods]
impl BoostStrengthMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::OneAndAHalf),
            2 => Ok(Self::Two),
            3 => Ok(Self::Five),
            4 => Ok(Self::Ten),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BoostStrengthMutator.{self:?}")
    }
}
