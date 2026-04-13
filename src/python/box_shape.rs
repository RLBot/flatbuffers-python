use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct BoxShape {
    pub length: Py<PyFloat>,
    pub width: Py<PyFloat>,
    pub height: Py<PyFloat>,
}

impl crate::PyDefault for BoxShape {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                length: crate::pyfloat_default(py),
                width: crate::pyfloat_default(py),
                height: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::BoxShape> for BoxShape {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::BoxShape) -> Self {
        BoxShape {
            length: crate::float_to_py(py, flat_t.length),
            width: crate::float_to_py(py, flat_t.width),
            height: crate::float_to_py(py, flat_t.height),
        }
    }
}

impl FromGil<&BoxShape> for flat::BoxShape {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &BoxShape) -> Self {
        Self {
            length: crate::float_from_py(py, &py_type.length),
            width: crate::float_from_py(py, &py_type.width),
            height: crate::float_from_py(py, &py_type.height),
        }
    }
}

#[pymethods]
impl BoxShape {
    #[new]
    #[pyo3(signature = (length=0.0, width=0.0, height=0.0))]
    pub fn new(py: Python, length: f64, width: f64, height: f64) -> Self {
        Self {
            length: PyFloat::new(py, length).unbind(),
            width: PyFloat::new(py, width).unbind(),
            height: PyFloat::new(py, height).unbind(),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "BoxShape(length={}, width={}, height={})",
            self.length, self.width, self.height,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("length", "width", "height")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::BoxShape::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::BoxShapeRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::BoxShape::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
