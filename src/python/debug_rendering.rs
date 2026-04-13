use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum DebugRendering {
    #[default]
    OffByDefault = 0,
    OnByDefault = 1,
    AlwaysOff = 2,
}

impl From<flat::DebugRendering> for DebugRendering {
    fn from(flat_t: flat::DebugRendering) -> Self {
        match flat_t {
            flat::DebugRendering::OffByDefault => Self::OffByDefault,
            flat::DebugRendering::OnByDefault => Self::OnByDefault,
            flat::DebugRendering::AlwaysOff => Self::AlwaysOff,
        }
    }
}

impl From<&DebugRendering> for flat::DebugRendering {
    fn from(py_type: &DebugRendering) -> Self {
        match py_type {
            DebugRendering::OffByDefault => Self::OffByDefault,
            DebugRendering::OnByDefault => Self::OnByDefault,
            DebugRendering::AlwaysOff => Self::AlwaysOff,
        }
    }
}

#[pymethods]
impl DebugRendering {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::OffByDefault),
            1 => Ok(Self::OnByDefault),
            2 => Ok(Self::AlwaysOff),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("DebugRendering.{self:?}")
    }
}
