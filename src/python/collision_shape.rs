use crate::{FromGil, PyDefault, flat};
use pyo3::prelude::*;

#[derive(pyo3::FromPyObject)]
pub enum CollisionShape {
    BoxShape(Py<super::BoxShape>),
    SphereShape(Py<super::SphereShape>),
    CylinderShape(Py<super::CylinderShape>),
}

impl CollisionShape {
    pub fn py_default(py: Python) -> Py<PyAny> {
        super::BoxShape::py_default(py).into_any()
    }
}

impl FromGil<&flat::CollisionShape> for CollisionShape {
    fn from_gil(py: Python, flat_t: &flat::CollisionShape) -> Self {
        match flat_t {
            flat::CollisionShape::BoxShape(item) => {
                Self::BoxShape(Py::new(py, super::BoxShape::from_gil(py, &**item)).unwrap())
            }
            flat::CollisionShape::SphereShape(item) => {
                Self::SphereShape(Py::new(py, super::SphereShape::from_gil(py, &**item)).unwrap())
            }
            flat::CollisionShape::CylinderShape(item) => Self::CylinderShape(
                Py::new(py, super::CylinderShape::from_gil(py, &**item)).unwrap(),
            ),
        }
    }
}

impl FromGil<&CollisionShape> for flat::CollisionShape {
    fn from_gil(py: Python, py_type: &CollisionShape) -> Self {
        match py_type {
            CollisionShape::BoxShape(item) => {
                flat::CollisionShape::BoxShape(Box::new(crate::from_py_into(py, item)))
            }
            CollisionShape::SphereShape(item) => {
                flat::CollisionShape::SphereShape(Box::new(crate::from_py_into(py, item)))
            }
            CollisionShape::CylinderShape(item) => {
                flat::CollisionShape::CylinderShape(Box::new(crate::from_py_into(py, item)))
            }
        }
    }
}

impl CollisionShape {
    pub fn into_any(self) -> Py<PyAny> {
        match self {
            Self::BoxShape(item) => item.into_any(),
            Self::SphereShape(item) => item.into_any(),
            Self::CylinderShape(item) => item.into_any(),
        }
    }

    pub fn __repr__(&self, py: Python) -> String {
        match self {
            Self::BoxShape(item) => item.borrow(py).__repr__(py),
            Self::SphereShape(item) => item.borrow(py).__repr__(py),
            Self::CylinderShape(item) => item.borrow(py).__repr__(py),
        }
    }
}
