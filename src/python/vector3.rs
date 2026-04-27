use crate::{FromGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
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

fn read_as_root<'a>(slice: &'a [u8]) -> ::planus::Result<flat::Vector3Ref<'a>> {
    planus::TableRead::from_buffer(
        planus::SliceWithStartOffset {
            buffer: slice,
            offset_from_start: 0,
        },
        0,
    )
    .map_err(|error_kind| error_kind.with_error_location("[Vector3Ref]", "read_as_root", 0))
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

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Vector3::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Vector3::from(flat_t_ref);

        Ok(crate::into_py_from(py, &flat_t))
    }
}
