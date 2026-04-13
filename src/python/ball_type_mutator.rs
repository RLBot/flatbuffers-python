use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::BallTypeMutator;

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
