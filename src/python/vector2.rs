use crate::{FromGil, PyDefault, flat};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct Vector2 {
    pub x: Py<PyFloat>,
    pub y: Py<PyFloat>,
}

impl crate::PyDefault for Vector2 {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                x: crate::pyfloat_default(py),
                y: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Vector2> for Vector2 {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Vector2) -> Self {
        Self {
            x: crate::float_to_py(py, flat_t.x),
            y: crate::float_to_py(py, flat_t.y),
        }
    }
}

impl FromGil<&Vector2> for flat::Vector2 {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Vector2) -> Self {
        Self {
            x: crate::float_from_py(py, &py_type.x),
            y: crate::float_from_py(py, &py_type.y),
        }
    }
}

#[pymethods]
impl Vector2 {
    #[new]
    #[pyo3(signature = (x=0.0, y=0.0))]
    pub fn new(py: Python, x: f64, y: f64) -> Self {
        Self {
            x: PyFloat::new(py, x).unbind(),
            y: PyFloat::new(py, y).unbind(),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!("Vector2(x={}, y={})", self.x, self.y,)
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("x", "y")
    }
}
