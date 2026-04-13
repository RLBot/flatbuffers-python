use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum Launcher {
    #[default]
    Steam = 0,
    Epic = 1,
    Custom = 2,
    NoLaunch = 3,
}

impl From<flat::Launcher> for Launcher {
    fn from(flat_t: flat::Launcher) -> Self {
        match flat_t {
            flat::Launcher::Steam => Self::Steam,
            flat::Launcher::Epic => Self::Epic,
            flat::Launcher::Custom => Self::Custom,
            flat::Launcher::NoLaunch => Self::NoLaunch,
        }
    }
}

impl From<&Launcher> for flat::Launcher {
    fn from(py_type: &Launcher) -> Self {
        match py_type {
            Launcher::Steam => Self::Steam,
            Launcher::Epic => Self::Epic,
            Launcher::Custom => Self::Custom,
            Launcher::NoLaunch => Self::NoLaunch,
        }
    }
}

#[pymethods]
impl Launcher {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Steam),
            1 => Ok(Self::Epic),
            2 => Ok(Self::Custom),
            3 => Ok(Self::NoLaunch),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("Launcher.{self:?}")
    }
}
