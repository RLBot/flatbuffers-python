use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct RemoveRenderGroup {
    #[pyo3(set)]
    pub id: i32,
}

impl crate::PyDefault for RemoveRenderGroup {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                id: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::RemoveRenderGroup> for RemoveRenderGroup {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::RemoveRenderGroup) -> Self {
        RemoveRenderGroup { id: flat_t.id }
    }
}

impl FromGil<&RemoveRenderGroup> for flat::RemoveRenderGroup {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &RemoveRenderGroup) -> Self {
        Self { id: py_type.id }
    }
}

#[pymethods]
impl RemoveRenderGroup {
    #[new]
    #[pyo3(signature = (id=0))]
    pub fn new(id: i32) -> Self {
        Self { id }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!("RemoveRenderGroup(id={})", self.id,)
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("id",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::RemoveRenderGroup::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::RemoveRenderGroupRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::RemoveRenderGroup::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
