use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum AssistGoalScoreMutator {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl From<flat::AssistGoalScoreMutator> for AssistGoalScoreMutator {
    fn from(flat_t: flat::AssistGoalScoreMutator) -> Self {
        match flat_t {
            flat::AssistGoalScoreMutator::Zero => Self::Zero,
            flat::AssistGoalScoreMutator::One => Self::One,
            flat::AssistGoalScoreMutator::Two => Self::Two,
            flat::AssistGoalScoreMutator::Three => Self::Three,
        }
    }
}

impl From<&AssistGoalScoreMutator> for flat::AssistGoalScoreMutator {
    fn from(py_type: &AssistGoalScoreMutator) -> Self {
        match py_type {
            AssistGoalScoreMutator::Zero => Self::Zero,
            AssistGoalScoreMutator::One => Self::One,
            AssistGoalScoreMutator::Two => Self::Two,
            AssistGoalScoreMutator::Three => Self::Three,
        }
    }
}

#[pymethods]
impl AssistGoalScoreMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("AssistGoalScoreMutator.{self:?}")
    }
}
