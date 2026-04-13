use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct SphereShape {
    pub diameter: Py<PyFloat>,
}

impl crate::PyDefault for SphereShape {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                diameter: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::SphereShape> for SphereShape {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::SphereShape) -> Self {
        SphereShape {
            diameter: crate::float_to_py(py, flat_t.diameter),
        }
    }
}

impl FromGil<&SphereShape> for flat::SphereShape {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &SphereShape) -> Self {
        Self {
            diameter: crate::float_from_py(py, &py_type.diameter),
        }
    }
}

#[pymethods]
impl SphereShape {
    #[new]
    #[pyo3(signature = (diameter=0.0))]
    pub fn new(py: Python, diameter: f64) -> Self {
        Self {
            diameter: PyFloat::new(py, diameter).unbind(),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!("SphereShape(diameter={})", self.diameter,)
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("diameter",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::SphereShape::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::SphereShapeRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::SphereShape::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
