use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::RumbleItem;

#[pymethods]
impl RumbleItem {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Boot),
            1 => Ok(Self::Disruptor),
            2 => Ok(Self::Freezer),
            3 => Ok(Self::Haymaker),
            4 => Ok(Self::Magnetizer),
            5 => Ok(Self::Plunger),
            6 => Ok(Self::Spike),
            7 => Ok(Self::Swapper),
            8 => Ok(Self::Tornado),
            9 => Ok(Self::GrapplingHook),
            10 => Ok(Self::PowerHitter),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("RumbleItem.{self:?}")
    }
}
