use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum DemolishScoreMutator {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl From<flat::DemolishScoreMutator> for DemolishScoreMutator {
    fn from(flat_t: flat::DemolishScoreMutator) -> Self {
        match flat_t {
            flat::DemolishScoreMutator::Zero => Self::Zero,
            flat::DemolishScoreMutator::One => Self::One,
            flat::DemolishScoreMutator::Two => Self::Two,
            flat::DemolishScoreMutator::Three => Self::Three,
        }
    }
}

impl From<&DemolishScoreMutator> for flat::DemolishScoreMutator {
    fn from(py_type: &DemolishScoreMutator) -> Self {
        match py_type {
            DemolishScoreMutator::Zero => Self::Zero,
            DemolishScoreMutator::One => Self::One,
            DemolishScoreMutator::Two => Self::Two,
            DemolishScoreMutator::Three => Self::Three,
        }
    }
}

#[pymethods]
impl DemolishScoreMutator {
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
        format!("DemolishScoreMutator.{self:?}")
    }
}
