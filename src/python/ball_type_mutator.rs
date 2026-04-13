use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum BallTypeMutator {
    #[default]
    Default = 0,
    Cube = 1,
    Puck = 2,
    Basketball = 3,
    Beachball = 4,
    Anniversary = 5,
    Haunted = 6,
    Ekin = 7,
    SpookyCube = 8,
    Egg = 9,
    PlayerSeeking = 10,
    Dropshot = 11,
    ScoreAbsorb = 12,
    Shoe = 13,
    PizzaPuck = 14,
}

impl From<flat::BallTypeMutator> for BallTypeMutator {
    fn from(flat_t: flat::BallTypeMutator) -> Self {
        match flat_t {
            flat::BallTypeMutator::Default => Self::Default,
            flat::BallTypeMutator::Cube => Self::Cube,
            flat::BallTypeMutator::Puck => Self::Puck,
            flat::BallTypeMutator::Basketball => Self::Basketball,
            flat::BallTypeMutator::Beachball => Self::Beachball,
            flat::BallTypeMutator::Anniversary => Self::Anniversary,
            flat::BallTypeMutator::Haunted => Self::Haunted,
            flat::BallTypeMutator::Ekin => Self::Ekin,
            flat::BallTypeMutator::SpookyCube => Self::SpookyCube,
            flat::BallTypeMutator::Egg => Self::Egg,
            flat::BallTypeMutator::PlayerSeeking => Self::PlayerSeeking,
            flat::BallTypeMutator::Dropshot => Self::Dropshot,
            flat::BallTypeMutator::ScoreAbsorb => Self::ScoreAbsorb,
            flat::BallTypeMutator::Shoe => Self::Shoe,
            flat::BallTypeMutator::PizzaPuck => Self::PizzaPuck,
        }
    }
}

impl From<&BallTypeMutator> for flat::BallTypeMutator {
    fn from(py_type: &BallTypeMutator) -> Self {
        match py_type {
            BallTypeMutator::Default => Self::Default,
            BallTypeMutator::Cube => Self::Cube,
            BallTypeMutator::Puck => Self::Puck,
            BallTypeMutator::Basketball => Self::Basketball,
            BallTypeMutator::Beachball => Self::Beachball,
            BallTypeMutator::Anniversary => Self::Anniversary,
            BallTypeMutator::Haunted => Self::Haunted,
            BallTypeMutator::Ekin => Self::Ekin,
            BallTypeMutator::SpookyCube => Self::SpookyCube,
            BallTypeMutator::Egg => Self::Egg,
            BallTypeMutator::PlayerSeeking => Self::PlayerSeeking,
            BallTypeMutator::Dropshot => Self::Dropshot,
            BallTypeMutator::ScoreAbsorb => Self::ScoreAbsorb,
            BallTypeMutator::Shoe => Self::Shoe,
            BallTypeMutator::PizzaPuck => Self::PizzaPuck,
        }
    }
}

#[pymethods]
impl BallTypeMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Cube),
            2 => Ok(Self::Puck),
            3 => Ok(Self::Basketball),
            4 => Ok(Self::Beachball),
            5 => Ok(Self::Anniversary),
            6 => Ok(Self::Haunted),
            7 => Ok(Self::Ekin),
            8 => Ok(Self::SpookyCube),
            9 => Ok(Self::Egg),
            10 => Ok(Self::PlayerSeeking),
            11 => Ok(Self::Dropshot),
            12 => Ok(Self::ScoreAbsorb),
            13 => Ok(Self::Shoe),
            14 => Ok(Self::PizzaPuck),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("BallTypeMutator.{self:?}")
    }
}
