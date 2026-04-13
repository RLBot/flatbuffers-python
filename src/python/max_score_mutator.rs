use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum MaxScoreMutator {
    #[default]
    Unlimited = 0,
    OneGoal = 1,
    ThreeGoals = 2,
    FiveGoals = 3,
    SevenGoals = 4,
    TenGoals = 5,
    TwentyGoals = 6,
    ThirtyGoals = 7,
    FortyGoals = 8,
    FiftyGoals = 9,
    SixtyGoals = 10,
    SeventyGoals = 11,
    EightyGoals = 12,
    NinetyGoals = 13,
    HundredGoals = 14,
}

impl From<flat::MaxScoreMutator> for MaxScoreMutator {
    fn from(flat_t: flat::MaxScoreMutator) -> Self {
        match flat_t {
            flat::MaxScoreMutator::Unlimited => Self::Unlimited,
            flat::MaxScoreMutator::OneGoal => Self::OneGoal,
            flat::MaxScoreMutator::ThreeGoals => Self::ThreeGoals,
            flat::MaxScoreMutator::FiveGoals => Self::FiveGoals,
            flat::MaxScoreMutator::SevenGoals => Self::SevenGoals,
            flat::MaxScoreMutator::TenGoals => Self::TenGoals,
            flat::MaxScoreMutator::TwentyGoals => Self::TwentyGoals,
            flat::MaxScoreMutator::ThirtyGoals => Self::ThirtyGoals,
            flat::MaxScoreMutator::FortyGoals => Self::FortyGoals,
            flat::MaxScoreMutator::FiftyGoals => Self::FiftyGoals,
            flat::MaxScoreMutator::SixtyGoals => Self::SixtyGoals,
            flat::MaxScoreMutator::SeventyGoals => Self::SeventyGoals,
            flat::MaxScoreMutator::EightyGoals => Self::EightyGoals,
            flat::MaxScoreMutator::NinetyGoals => Self::NinetyGoals,
            flat::MaxScoreMutator::HundredGoals => Self::HundredGoals,
        }
    }
}

impl From<&MaxScoreMutator> for flat::MaxScoreMutator {
    fn from(py_type: &MaxScoreMutator) -> Self {
        match py_type {
            MaxScoreMutator::Unlimited => Self::Unlimited,
            MaxScoreMutator::OneGoal => Self::OneGoal,
            MaxScoreMutator::ThreeGoals => Self::ThreeGoals,
            MaxScoreMutator::FiveGoals => Self::FiveGoals,
            MaxScoreMutator::SevenGoals => Self::SevenGoals,
            MaxScoreMutator::TenGoals => Self::TenGoals,
            MaxScoreMutator::TwentyGoals => Self::TwentyGoals,
            MaxScoreMutator::ThirtyGoals => Self::ThirtyGoals,
            MaxScoreMutator::FortyGoals => Self::FortyGoals,
            MaxScoreMutator::FiftyGoals => Self::FiftyGoals,
            MaxScoreMutator::SixtyGoals => Self::SixtyGoals,
            MaxScoreMutator::SeventyGoals => Self::SeventyGoals,
            MaxScoreMutator::EightyGoals => Self::EightyGoals,
            MaxScoreMutator::NinetyGoals => Self::NinetyGoals,
            MaxScoreMutator::HundredGoals => Self::HundredGoals,
        }
    }
}

#[pymethods]
impl MaxScoreMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Unlimited),
            1 => Ok(Self::OneGoal),
            2 => Ok(Self::ThreeGoals),
            3 => Ok(Self::FiveGoals),
            4 => Ok(Self::SevenGoals),
            5 => Ok(Self::TenGoals),
            6 => Ok(Self::TwentyGoals),
            7 => Ok(Self::ThirtyGoals),
            8 => Ok(Self::FortyGoals),
            9 => Ok(Self::FiftyGoals),
            10 => Ok(Self::SixtyGoals),
            11 => Ok(Self::SeventyGoals),
            12 => Ok(Self::EightyGoals),
            13 => Ok(Self::NinetyGoals),
            14 => Ok(Self::HundredGoals),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("MaxScoreMutator.{self:?}")
    }
}
