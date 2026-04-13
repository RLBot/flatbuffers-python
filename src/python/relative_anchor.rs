use crate::{FromGil, PyDefault, flat};
use pyo3::prelude::*;

#[derive(pyo3::FromPyObject)]
pub enum RelativeAnchor {
    BallAnchor(Py<super::BallAnchor>),
    CarAnchor(Py<super::CarAnchor>),
}

impl RelativeAnchor {
    pub fn py_default(py: Python) -> Py<PyAny> {
        super::BallAnchor::py_default(py).into_any()
    }
}

impl FromGil<&flat::RelativeAnchor> for RelativeAnchor {
    fn from_gil(py: Python, flat_t: &flat::RelativeAnchor) -> Self {
        match flat_t {
            flat::RelativeAnchor::BallAnchor(item) => {
                Self::BallAnchor(Py::new(py, super::BallAnchor::from_gil(py, &**item)).unwrap())
            }
            flat::RelativeAnchor::CarAnchor(item) => {
                Self::CarAnchor(Py::new(py, super::CarAnchor::from_gil(py, &**item)).unwrap())
            }
        }
    }
}

impl FromGil<&RelativeAnchor> for flat::RelativeAnchor {
    fn from_gil(py: Python, py_type: &RelativeAnchor) -> Self {
        match py_type {
            RelativeAnchor::BallAnchor(item) => {
                flat::RelativeAnchor::BallAnchor(Box::new(crate::from_py_into(py, item)))
            }
            RelativeAnchor::CarAnchor(item) => {
                flat::RelativeAnchor::CarAnchor(Box::new(crate::from_py_into(py, item)))
            }
        }
    }
}

impl RelativeAnchor {
    pub fn into_any(self) -> Py<PyAny> {
        match self {
            Self::BallAnchor(item) => item.into_any(),
            Self::CarAnchor(item) => item.into_any(),
        }
    }

    pub fn __repr__(&self, py: Python) -> String {
        match self {
            Self::BallAnchor(item) => item.borrow(py).__repr__(py),
            Self::CarAnchor(item) => item.borrow(py).__repr__(py),
        }
    }
}
