use crate::{FromGil, PyDefault, flat};
use pyo3::prelude::*;

#[derive(pyo3::FromPyObject)]
pub enum RenderType {
    Line3D(Py<super::Line3D>),
    PolyLine3D(Py<super::PolyLine3D>),
    String2D(Py<super::String2D>),
    String3D(Py<super::String3D>),
    Rect2D(Py<super::Rect2D>),
    Rect3D(Py<super::Rect3D>),
}

impl RenderType {
    pub fn py_default(py: Python) -> Py<PyAny> {
        super::Line3D::py_default(py).into_any()
    }
}

impl FromGil<&flat::RenderType> for RenderType {
    fn from_gil(py: Python, flat_t: &flat::RenderType) -> Self {
        match flat_t {
            flat::RenderType::Line3D(item) => {
                Self::Line3D(Py::new(py, super::Line3D::from_gil(py, &**item)).unwrap())
            }
            flat::RenderType::PolyLine3D(item) => {
                Self::PolyLine3D(Py::new(py, super::PolyLine3D::from_gil(py, &**item)).unwrap())
            }
            flat::RenderType::String2D(item) => {
                Self::String2D(Py::new(py, super::String2D::from_gil(py, &**item)).unwrap())
            }
            flat::RenderType::String3D(item) => {
                Self::String3D(Py::new(py, super::String3D::from_gil(py, &**item)).unwrap())
            }
            flat::RenderType::Rect2D(item) => {
                Self::Rect2D(Py::new(py, super::Rect2D::from_gil(py, &**item)).unwrap())
            }
            flat::RenderType::Rect3D(item) => {
                Self::Rect3D(Py::new(py, super::Rect3D::from_gil(py, &**item)).unwrap())
            }
        }
    }
}

impl FromGil<&RenderType> for flat::RenderType {
    fn from_gil(py: Python, py_type: &RenderType) -> Self {
        match py_type {
            RenderType::Line3D(item) => {
                flat::RenderType::Line3D(Box::new(crate::from_py_into(py, item)))
            }
            RenderType::PolyLine3D(item) => {
                flat::RenderType::PolyLine3D(Box::new(crate::from_py_into(py, item)))
            }
            RenderType::String2D(item) => {
                flat::RenderType::String2D(Box::new(crate::from_py_into(py, item)))
            }
            RenderType::String3D(item) => {
                flat::RenderType::String3D(Box::new(crate::from_py_into(py, item)))
            }
            RenderType::Rect2D(item) => {
                flat::RenderType::Rect2D(Box::new(crate::from_py_into(py, item)))
            }
            RenderType::Rect3D(item) => {
                flat::RenderType::Rect3D(Box::new(crate::from_py_into(py, item)))
            }
        }
    }
}

impl RenderType {
    pub fn into_any(self) -> Py<PyAny> {
        match self {
            Self::Line3D(item) => item.into_any(),
            Self::PolyLine3D(item) => item.into_any(),
            Self::String2D(item) => item.into_any(),
            Self::String3D(item) => item.into_any(),
            Self::Rect2D(item) => item.into_any(),
            Self::Rect3D(item) => item.into_any(),
        }
    }

    pub fn __repr__(&self, py: Python) -> String {
        match self {
            Self::Line3D(item) => item.borrow(py).__repr__(py),
            Self::PolyLine3D(item) => item.borrow(py).__repr__(py),
            Self::String2D(item) => item.borrow(py).__repr__(py),
            Self::String3D(item) => item.borrow(py).__repr__(py),
            Self::Rect2D(item) => item.borrow(py).__repr__(py),
            Self::Rect3D(item) => item.borrow(py).__repr__(py),
        }
    }
}
