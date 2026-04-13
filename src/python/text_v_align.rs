use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum TextVAlign {
    #[default]
    Top = 0,
    Center = 1,
    Bottom = 2,
}

impl From<flat::TextVAlign> for TextVAlign {
    fn from(flat_t: flat::TextVAlign) -> Self {
        match flat_t {
            flat::TextVAlign::Top => Self::Top,
            flat::TextVAlign::Center => Self::Center,
            flat::TextVAlign::Bottom => Self::Bottom,
        }
    }
}

impl From<&TextVAlign> for flat::TextVAlign {
    fn from(py_type: &TextVAlign) -> Self {
        match py_type {
            TextVAlign::Top => Self::Top,
            TextVAlign::Center => Self::Center,
            TextVAlign::Bottom => Self::Bottom,
        }
    }
}

#[pymethods]
impl TextVAlign {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Top),
            1 => Ok(Self::Center),
            2 => Ok(Self::Bottom),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("TextVAlign.{self:?}")
    }
}
