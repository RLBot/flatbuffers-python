use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct PolyLine3D {
    #[pyo3(set)]
    pub points: Py<PyList>,
    #[pyo3(set)]
    pub color: Py<super::Color>,
}

impl crate::PyDefault for PolyLine3D {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                points: PyList::empty(py).unbind(),
                color: super::Color::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PolyLine3D> for PolyLine3D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PolyLine3D) -> Self {
        PolyLine3D {
            points: PyList::new(
                py,
                flat_t
                    .points
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::Vector3>(py, x)),
            )
            .unwrap()
            .unbind(),
            color: crate::into_py_from(py, &flat_t.color),
        }
    }
}

impl FromGil<&PolyLine3D> for flat::PolyLine3D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PolyLine3D) -> Self {
        Self {
            points: py_type
                .points
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            color: crate::from_py_into(py, &py_type.color),
        }
    }
}

#[pymethods]
impl PolyLine3D {
    #[new]
    #[pyo3(signature = (points=None, color=None))]
    pub fn new(py: Python, points: Option<Py<PyList>>, color: Option<Py<super::Color>>) -> Self {
        Self {
            points: points.unwrap_or_else(|| PyList::empty(py).unbind()),
            color: color.unwrap_or_else(|| super::Color::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "PolyLine3D(points=[{}], color={})",
            self.points
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::Vector3>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.color.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("points", "color")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PolyLine3D::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::PolyLine3DRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PolyLine3D::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
