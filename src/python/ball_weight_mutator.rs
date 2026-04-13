use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BallWeightMutator {
    #[default]
    Default = 0,
    Light = 1,
    Heavy = 2,
    SuperLight = 3,
    CurveBall = 4,
    BeachBallCurve = 5,
    MagnusFutBall = 6,
    MagnusFutballLess = 7,
}

impl From<flat::BallWeightMutator> for BallWeightMutator {
    fn from(flat_t: flat::BallWeightMutator) -> Self {
        match flat_t {
            flat::BallWeightMutator::Default => Self::Default,
            flat::BallWeightMutator::Light => Self::Light,
            flat::BallWeightMutator::Heavy => Self::Heavy,
            flat::BallWeightMutator::SuperLight => Self::SuperLight,
            flat::BallWeightMutator::CurveBall => Self::CurveBall,
            flat::BallWeightMutator::BeachBallCurve => Self::BeachBallCurve,
            flat::BallWeightMutator::MagnusFutBall => Self::MagnusFutBall,
            flat::BallWeightMutator::MagnusFutballLess => Self::MagnusFutballLess,
        }
    }
}

impl From<&BallWeightMutator> for flat::BallWeightMutator {
    fn from(py_type: &BallWeightMutator) -> Self {
        match py_type {
            BallWeightMutator::Default => Self::Default,
            BallWeightMutator::Light => Self::Light,
            BallWeightMutator::Heavy => Self::Heavy,
            BallWeightMutator::SuperLight => Self::SuperLight,
            BallWeightMutator::CurveBall => Self::CurveBall,
            BallWeightMutator::BeachBallCurve => Self::BeachBallCurve,
            BallWeightMutator::MagnusFutBall => Self::MagnusFutBall,
            BallWeightMutator::MagnusFutballLess => Self::MagnusFutballLess,
        }
    }
}

#[pymethods]
impl BallWeightMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Light),
            2 => Ok(Self::Heavy),
            3 => Ok(Self::SuperLight),
            4 => Ok(Self::CurveBall),
            5 => Ok(Self::BeachBallCurve),
            6 => Ok(Self::MagnusFutBall),
            7 => Ok(Self::MagnusFutballLess),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BallWeightMutator.{self:?}")
    }
}
