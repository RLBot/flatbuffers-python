use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct Line3D {
    #[pyo3(set)]
    pub start: Py<super::RenderAnchor>,
    #[pyo3(set)]
    pub end: Py<super::RenderAnchor>,
    #[pyo3(set)]
    pub color: Py<super::Color>,
}

impl crate::PyDefault for Line3D {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                start: super::RenderAnchor::py_default(py),
                end: super::RenderAnchor::py_default(py),
                color: super::Color::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Line3D> for Line3D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Line3D) -> Self {
        Line3D {
            start: crate::into_py_from(py, &*flat_t.start),
            end: crate::into_py_from(py, &*flat_t.end),
            color: crate::into_py_from(py, &flat_t.color),
        }
    }
}

impl FromGil<&Line3D> for flat::Line3D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Line3D) -> Self {
        Self {
            start: Box::new(crate::from_py_into(py, &py_type.start)),
            end: Box::new(crate::from_py_into(py, &py_type.end)),
            color: crate::from_py_into(py, &py_type.color),
        }
    }
}

#[pymethods]
impl Line3D {
    #[new]
    #[pyo3(signature = (start=None, end=None, color=None))]
    pub fn new(
        py: Python,
        start: Option<Py<super::RenderAnchor>>,
        end: Option<Py<super::RenderAnchor>>,
        color: Option<Py<super::Color>>,
    ) -> Self {
        Self {
            start: start.unwrap_or_else(|| super::RenderAnchor::py_default(py)),
            end: end.unwrap_or_else(|| super::RenderAnchor::py_default(py)),
            color: color.unwrap_or_else(|| super::Color::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Line3D(start={}, end={}, color={})",
            self.start.borrow(py).__repr__(py),
            self.end.borrow(py).__repr__(py),
            self.color.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("start", "end", "color")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Line3D::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::Line3DRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Line3D::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
