use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct RenderGroup {
    #[pyo3(set)]
    pub render_messages: Py<PyList>,
    #[pyo3(set)]
    pub id: i32,
}

impl crate::PyDefault for RenderGroup {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                render_messages: PyList::empty(py).unbind(),
                id: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::RenderGroup> for RenderGroup {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::RenderGroup) -> Self {
        RenderGroup {
            render_messages: PyList::new(
                py,
                flat_t
                    .render_messages
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::RenderMessage>(py, x)),
            )
            .unwrap()
            .unbind(),
            id: flat_t.id,
        }
    }
}

impl FromGil<&RenderGroup> for flat::RenderGroup {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &RenderGroup) -> Self {
        Self {
            render_messages: py_type
                .render_messages
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            id: py_type.id,
        }
    }
}

#[pymethods]
impl RenderGroup {
    #[new]
    #[pyo3(signature = (render_messages=None, id=0))]
    pub fn new(py: Python, render_messages: Option<Py<PyList>>, id: i32) -> Self {
        Self {
            render_messages: render_messages.unwrap_or_else(|| PyList::empty(py).unbind()),
            id,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "RenderGroup(render_messages=[{}], id={})",
            self.render_messages
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::RenderMessage>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.id,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("render_messages", "id")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::RenderGroup::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::RenderGroupRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::RenderGroup::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
