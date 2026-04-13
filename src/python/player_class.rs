use crate::{FromGil, PyDefault, flat};
use pyo3::prelude::*;

#[derive(pyo3::FromPyObject)]
pub enum PlayerClass {
    Human(Py<super::Human>),
    CustomBot(Py<super::CustomBot>),
    PsyonixBot(Py<super::PsyonixBot>),
}

impl PlayerClass {
    pub fn py_default(py: Python) -> Py<PyAny> {
        super::Human::py_default(py).into_any()
    }
}

impl FromGil<&flat::PlayerClass> for PlayerClass {
    fn from_gil(py: Python, flat_t: &flat::PlayerClass) -> Self {
        match flat_t {
            flat::PlayerClass::Human(item) => {
                Self::Human(Py::new(py, super::Human::from_gil(py, &**item)).unwrap())
            }
            flat::PlayerClass::CustomBot(item) => {
                Self::CustomBot(Py::new(py, super::CustomBot::from_gil(py, &**item)).unwrap())
            }
            flat::PlayerClass::PsyonixBot(item) => {
                Self::PsyonixBot(Py::new(py, super::PsyonixBot::from_gil(py, &**item)).unwrap())
            }
        }
    }
}

impl FromGil<&PlayerClass> for flat::PlayerClass {
    fn from_gil(py: Python, py_type: &PlayerClass) -> Self {
        match py_type {
            PlayerClass::Human(item) => {
                flat::PlayerClass::Human(Box::new(crate::from_py_into(py, item)))
            }
            PlayerClass::CustomBot(item) => {
                flat::PlayerClass::CustomBot(Box::new(crate::from_py_into(py, item)))
            }
            PlayerClass::PsyonixBot(item) => {
                flat::PlayerClass::PsyonixBot(Box::new(crate::from_py_into(py, item)))
            }
        }
    }
}

impl PlayerClass {
    pub fn into_any(self) -> Py<PyAny> {
        match self {
            Self::Human(item) => item.into_any(),
            Self::CustomBot(item) => item.into_any(),
            Self::PsyonixBot(item) => item.into_any(),
        }
    }

    pub fn __repr__(&self, py: Python) -> String {
        match self {
            Self::Human(item) => item.borrow(py).__repr__(py),
            Self::CustomBot(item) => item.borrow(py).__repr__(py),
            Self::PsyonixBot(item) => item.borrow(py).__repr__(py),
        }
    }
}
