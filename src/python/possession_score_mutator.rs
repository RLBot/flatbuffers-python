use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum PossessionScoreMutator {
    #[default]
    Off = 0,
    OneSecond = 1,
    TwoSeconds = 2,
    ThreeSeconds = 3,
}

impl From<flat::PossessionScoreMutator> for PossessionScoreMutator {
    fn from(flat_t: flat::PossessionScoreMutator) -> Self {
        match flat_t {
            flat::PossessionScoreMutator::Off => Self::Off,
            flat::PossessionScoreMutator::OneSecond => Self::OneSecond,
            flat::PossessionScoreMutator::TwoSeconds => Self::TwoSeconds,
            flat::PossessionScoreMutator::ThreeSeconds => Self::ThreeSeconds,
        }
    }
}

impl From<&PossessionScoreMutator> for flat::PossessionScoreMutator {
    fn from(py_type: &PossessionScoreMutator) -> Self {
        match py_type {
            PossessionScoreMutator::Off => Self::Off,
            PossessionScoreMutator::OneSecond => Self::OneSecond,
            PossessionScoreMutator::TwoSeconds => Self::TwoSeconds,
            PossessionScoreMutator::ThreeSeconds => Self::ThreeSeconds,
        }
    }
}

#[pymethods]
impl PossessionScoreMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::OneSecond),
            2 => Ok(Self::TwoSeconds),
            3 => Ok(Self::ThreeSeconds),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("PossessionScoreMutator.{self:?}")
    }
}
