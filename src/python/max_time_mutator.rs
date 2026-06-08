use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::MaxTimeMutator;

#[pymethods]
impl MaxTimeMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Unlimited),
            1 => Ok(Self::OneMinute),
            2 => Ok(Self::TwoMinutes),
            3 => Ok(Self::ThreeMinutes),
            4 => Ok(Self::FourMinutes),
            5 => Ok(Self::FiveMinutes),
            6 => Ok(Self::SixMinutes),
            7 => Ok(Self::SevenMinutes),
            8 => Ok(Self::EightMinutes),
            9 => Ok(Self::NineMinutes),
            10 => Ok(Self::TenMinutes),
            11 => Ok(Self::ElevenMinutes),
            12 => Ok(Self::TwelveMinutes),
            13 => Ok(Self::ThirteenMinutes),
            14 => Ok(Self::FourteenMinutes),
            15 => Ok(Self::FifteenMinutes),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("MaxTimeMutator.{self:?}")
    }
}
