use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct CylinderShape {
    pub diameter: Py<PyFloat>,
    pub height: Py<PyFloat>,
}

impl crate::PyDefault for CylinderShape {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                diameter: crate::pyfloat_default(py),
                height: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::CylinderShape> for CylinderShape {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::CylinderShape) -> Self {
        CylinderShape {
            diameter: crate::float_to_py(py, flat_t.diameter),
            height: crate::float_to_py(py, flat_t.height),
        }
    }
}

impl FromGil<&CylinderShape> for flat::CylinderShape {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &CylinderShape) -> Self {
        Self {
            diameter: crate::float_from_py(py, &py_type.diameter),
            height: crate::float_from_py(py, &py_type.height),
        }
    }
}

#[pymethods]
impl CylinderShape {
    #[new]
    #[pyo3(signature = (diameter=0.0, height=0.0))]
    pub fn new(py: Python, diameter: f64, height: f64) -> Self {
        Self {
            diameter: PyFloat::new(py, diameter).unbind(),
            height: PyFloat::new(py, height).unbind(),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "CylinderShape(diameter={}, height={})",
            self.diameter, self.height,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("diameter", "height")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::CylinderShape::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::CylinderShapeRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::CylinderShape::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
