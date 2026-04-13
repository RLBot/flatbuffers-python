use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum SeriesLengthMutator {
    #[default]
    Unlimited = 0,
    ThreeGames = 1,
    FiveGames = 2,
    SevenGames = 3,
}

impl From<flat::SeriesLengthMutator> for SeriesLengthMutator {
    fn from(flat_t: flat::SeriesLengthMutator) -> Self {
        match flat_t {
            flat::SeriesLengthMutator::Unlimited => Self::Unlimited,
            flat::SeriesLengthMutator::ThreeGames => Self::ThreeGames,
            flat::SeriesLengthMutator::FiveGames => Self::FiveGames,
            flat::SeriesLengthMutator::SevenGames => Self::SevenGames,
        }
    }
}

impl From<&SeriesLengthMutator> for flat::SeriesLengthMutator {
    fn from(py_type: &SeriesLengthMutator) -> Self {
        match py_type {
            SeriesLengthMutator::Unlimited => Self::Unlimited,
            SeriesLengthMutator::ThreeGames => Self::ThreeGames,
            SeriesLengthMutator::FiveGames => Self::FiveGames,
            SeriesLengthMutator::SevenGames => Self::SevenGames,
        }
    }
}

#[pymethods]
impl SeriesLengthMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Unlimited),
            1 => Ok(Self::ThreeGames),
            2 => Ok(Self::FiveGames),
            3 => Ok(Self::SevenGames),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("SeriesLengthMutator.{self:?}")
    }
}
