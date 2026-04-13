use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct RenderAnchor {
    #[pyo3(set)]
    pub world: Py<super::Vector3>,
    #[pyo3(set)]
    pub relative: Option<Py<PyAny>>,
}

impl crate::PyDefault for RenderAnchor {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                world: super::Vector3::py_default(py),
                relative: None,
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::RenderAnchor> for RenderAnchor {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::RenderAnchor) -> Self {
        RenderAnchor {
            world: crate::into_py_from(py, &flat_t.world),
            relative: flat_t
                .relative
                .as_ref()
                .map(|x| IntoGil::<super::RelativeAnchor>::into_gil(x, py).into_any()),
        }
    }
}

impl FromGil<&RenderAnchor> for flat::RenderAnchor {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &RenderAnchor) -> Self {
        Self {
            world: crate::from_py_into(py, &py_type.world),
            relative: py_type.relative.as_ref().map(|x| {
                super::RelativeAnchor::extract(x.bind_borrowed(py))
                    .as_ref()
                    .unwrap()
                    .into_gil(py)
            }),
        }
    }
}

#[pymethods]
impl RenderAnchor {
    #[new]
    #[pyo3(signature = (world=None, relative=None))]
    pub fn new(py: Python, world: Option<Py<super::Vector3>>, relative: Option<Py<PyAny>>) -> Self {
        Self {
            world: world.unwrap_or_else(|| super::Vector3::py_default(py)),
            relative,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "RenderAnchor(world={}, relative={})",
            self.world.borrow(py).__repr__(py),
            self.relative.as_ref().map_or_else(crate::none_str, |i| {
                super::RelativeAnchor::extract(i.bind_borrowed(py))
                    .unwrap()
                    .__repr__(py)
            }),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("world", "relative")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::RenderAnchor::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::RenderAnchorRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::RenderAnchor::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
