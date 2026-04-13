use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::GameMode;

#[pymethods]
impl GameMode {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Soccar),
            1 => Ok(Self::Hoops),
            2 => Ok(Self::Dropshot),
            3 => Ok(Self::Snowday),
            4 => Ok(Self::Rumble),
            5 => Ok(Self::Heatseeker),
            6 => Ok(Self::Gridiron),
            7 => Ok(Self::Knockout),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("GameMode.{self:?}")
    }
}
