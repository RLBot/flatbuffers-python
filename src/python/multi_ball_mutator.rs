use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum MultiBallMutator {
    #[default]
    One = 0,
    Two = 1,
    Four = 2,
    Six = 3,
}

impl From<flat::MultiBallMutator> for MultiBallMutator {
    fn from(flat_t: flat::MultiBallMutator) -> Self {
        match flat_t {
            flat::MultiBallMutator::One => Self::One,
            flat::MultiBallMutator::Two => Self::Two,
            flat::MultiBallMutator::Four => Self::Four,
            flat::MultiBallMutator::Six => Self::Six,
        }
    }
}

impl From<&MultiBallMutator> for flat::MultiBallMutator {
    fn from(py_type: &MultiBallMutator) -> Self {
        match py_type {
            MultiBallMutator::One => Self::One,
            MultiBallMutator::Two => Self::Two,
            MultiBallMutator::Four => Self::Four,
            MultiBallMutator::Six => Self::Six,
        }
    }
}

#[pymethods]
impl MultiBallMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            2 => Ok(Self::Four),
            3 => Ok(Self::Six),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("MultiBallMutator.{self:?}")
    }
}
