use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum TextHAlign {
    #[default]
    Left = 0,
    Center = 1,
    Right = 2,
}

impl From<flat::TextHAlign> for TextHAlign {
    fn from(flat_t: flat::TextHAlign) -> Self {
        match flat_t {
            flat::TextHAlign::Left => Self::Left,
            flat::TextHAlign::Center => Self::Center,
            flat::TextHAlign::Right => Self::Right,
        }
    }
}

impl From<&TextHAlign> for flat::TextHAlign {
    fn from(py_type: &TextHAlign) -> Self {
        match py_type {
            TextHAlign::Left => Self::Left,
            TextHAlign::Center => Self::Center,
            TextHAlign::Right => Self::Right,
        }
    }
}

#[pymethods]
impl TextHAlign {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Left),
            1 => Ok(Self::Center),
            2 => Ok(Self::Right),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("TextHAlign.{self:?}")
    }
}
