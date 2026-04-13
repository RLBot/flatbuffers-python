use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct Vector3Partial {
    pub x: Option<Py<PyFloat>>,
    pub y: Option<Py<PyFloat>>,
    pub z: Option<Py<PyFloat>>,
}

impl crate::PyDefault for Vector3Partial {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                x: None,
                y: None,
                z: None,
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Vector3Partial> for Vector3Partial {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Vector3Partial) -> Self {
        Vector3Partial {
            x: flat_t.x.map(|x| crate::float_to_py(py, x.val)),
            y: flat_t.y.map(|x| crate::float_to_py(py, x.val)),
            z: flat_t.z.map(|x| crate::float_to_py(py, x.val)),
        }
    }
}

impl FromGil<&Vector3Partial> for flat::Vector3Partial {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Vector3Partial) -> Self {
        Self {
            x: py_type.x.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
            y: py_type.y.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
            z: py_type.z.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
        }
    }
}

#[pymethods]
impl Vector3Partial {
    #[new]
    #[pyo3(signature = (x=None, y=None, z=None))]
    pub fn new(py: Python, x: Option<f64>, y: Option<f64>, z: Option<f64>) -> Self {
        Self {
            x: x.map(|x| PyFloat::new(py, x).unbind()),
            y: y.map(|x| PyFloat::new(py, x).unbind()),
            z: z.map(|x| PyFloat::new(py, x).unbind()),
        }
    }

    #[setter]
    pub fn x(&mut self, py: Python, value: Option<f64>) {
        self.x = value.map(|x| PyFloat::new(py, x).unbind());
    }

    #[setter]
    pub fn y(&mut self, py: Python, value: Option<f64>) {
        self.y = value.map(|x| PyFloat::new(py, x).unbind());
    }

    #[setter]
    pub fn z(&mut self, py: Python, value: Option<f64>) {
        self.z = value.map(|x| PyFloat::new(py, x).unbind());
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Vector3Partial(x={}, y={}, z={})",
            self.x
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
            self.y
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
            self.z
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("x", "y", "z")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Vector3Partial::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::Vector3PartialRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Vector3Partial::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
