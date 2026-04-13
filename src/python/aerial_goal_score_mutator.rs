use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum AerialGoalScoreMutator {
    #[default]
    One = 0,
    Zero = 1,
    Two = 2,
    Three = 3,
    Five = 4,
    Ten = 5,
}

impl From<flat::AerialGoalScoreMutator> for AerialGoalScoreMutator {
    fn from(flat_t: flat::AerialGoalScoreMutator) -> Self {
        match flat_t {
            flat::AerialGoalScoreMutator::One => Self::One,
            flat::AerialGoalScoreMutator::Zero => Self::Zero,
            flat::AerialGoalScoreMutator::Two => Self::Two,
            flat::AerialGoalScoreMutator::Three => Self::Three,
            flat::AerialGoalScoreMutator::Five => Self::Five,
            flat::AerialGoalScoreMutator::Ten => Self::Ten,
        }
    }
}

impl From<&AerialGoalScoreMutator> for flat::AerialGoalScoreMutator {
    fn from(py_type: &AerialGoalScoreMutator) -> Self {
        match py_type {
            AerialGoalScoreMutator::One => Self::One,
            AerialGoalScoreMutator::Zero => Self::Zero,
            AerialGoalScoreMutator::Two => Self::Two,
            AerialGoalScoreMutator::Three => Self::Three,
            AerialGoalScoreMutator::Five => Self::Five,
            AerialGoalScoreMutator::Ten => Self::Ten,
        }
    }
}

#[pymethods]
impl AerialGoalScoreMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::Zero),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Five),
            5 => Ok(Self::Ten),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("AerialGoalScoreMutator.{self:?}")
    }
}
