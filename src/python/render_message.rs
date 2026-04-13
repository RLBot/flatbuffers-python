use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct RenderMessage {
    #[pyo3(set)]
    pub variety: Py<PyAny>,
}

impl crate::PyDefault for RenderMessage {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                variety: super::RenderType::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::RenderMessage> for RenderMessage {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::RenderMessage) -> Self {
        RenderMessage {
            variety: IntoGil::<super::RenderType>::into_gil(&flat_t.variety, py).into_any(),
        }
    }
}

impl FromGil<&RenderMessage> for flat::RenderMessage {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &RenderMessage) -> Self {
        Self {
            variety: super::RenderType::extract(py_type.variety.bind_borrowed(py))
                .as_ref()
                .unwrap()
                .into_gil(py),
        }
    }
}

#[pymethods]
impl RenderMessage {
    #[new]
    #[pyo3(signature = (variety=None))]
    pub fn new(py: Python, variety: Option<Py<PyAny>>) -> Self {
        Self {
            variety: variety.unwrap_or_else(|| super::RenderType::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "RenderMessage(variety={})",
            super::RenderType::extract(self.variety.bind_borrowed(py))
                .unwrap()
                .__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("variety",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::RenderMessage::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::RenderMessageRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::RenderMessage::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
