use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum NormalGoalScoreMutator {
    #[default]
    One = 0,
    Zero = 1,
    Two = 2,
    Three = 3,
    Five = 4,
    Ten = 5,
}

impl From<flat::NormalGoalScoreMutator> for NormalGoalScoreMutator {
    fn from(flat_t: flat::NormalGoalScoreMutator) -> Self {
        match flat_t {
            flat::NormalGoalScoreMutator::One => Self::One,
            flat::NormalGoalScoreMutator::Zero => Self::Zero,
            flat::NormalGoalScoreMutator::Two => Self::Two,
            flat::NormalGoalScoreMutator::Three => Self::Three,
            flat::NormalGoalScoreMutator::Five => Self::Five,
            flat::NormalGoalScoreMutator::Ten => Self::Ten,
        }
    }
}

impl From<&NormalGoalScoreMutator> for flat::NormalGoalScoreMutator {
    fn from(py_type: &NormalGoalScoreMutator) -> Self {
        match py_type {
            NormalGoalScoreMutator::One => Self::One,
            NormalGoalScoreMutator::Zero => Self::Zero,
            NormalGoalScoreMutator::Two => Self::Two,
            NormalGoalScoreMutator::Three => Self::Three,
            NormalGoalScoreMutator::Five => Self::Five,
            NormalGoalScoreMutator::Ten => Self::Ten,
        }
    }
}

#[pymethods]
impl NormalGoalScoreMutator {
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
        format!("NormalGoalScoreMutator.{self:?}")
    }
}
