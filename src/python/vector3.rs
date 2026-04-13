use crate::{FromGil, PyDefault, flat};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct Vector3 {
    pub x: Py<PyFloat>,
    pub y: Py<PyFloat>,
    pub z: Py<PyFloat>,
}

impl crate::PyDefault for Vector3 {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                x: crate::pyfloat_default(py),
                y: crate::pyfloat_default(py),
                z: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Vector3> for Vector3 {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Vector3) -> Self {
        Self {
            x: crate::float_to_py(py, flat_t.x),
            y: crate::float_to_py(py, flat_t.y),
            z: crate::float_to_py(py, flat_t.z),
        }
    }
}

impl FromGil<&Vector3> for flat::Vector3 {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Vector3) -> Self {
        Self {
            x: crate::float_from_py(py, &py_type.x),
            y: crate::float_from_py(py, &py_type.y),
            z: crate::float_from_py(py, &py_type.z),
        }
    }
}

#[pymethods]
impl Vector3 {
    #[new]
    #[pyo3(signature = (x=0.0, y=0.0, z=0.0))]
    pub fn new(py: Python, x: f64, y: f64, z: f64) -> Self {
        Self {
            x: PyFloat::new(py, x).unbind(),
            y: PyFloat::new(py, y).unbind(),
            z: PyFloat::new(py, z).unbind(),
        }
    }

    #[setter]
    pub fn x(&mut self, py: Python, value: f64) {
        self.x = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn y(&mut self, py: Python, value: f64) {
        self.y = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn z(&mut self, py: Python, value: f64) {
        self.z = PyFloat::new(py, value).unbind();
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!("Vector3(x={}, y={}, z={})", self.x, self.y, self.z,)
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("x", "y", "z")
    }
}
