use crate::{FromGil, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen)]
#[derive(Default)]
pub struct InitComplete {}

impl From<&flat::InitComplete> for InitComplete {
    fn from(_: &flat::InitComplete) -> Self {
        InitComplete {}
    }
}

impl From<&InitComplete> for flat::InitComplete {
    fn from(_: &InitComplete) -> Self {
        Self {}
    }
}

#[pymethods]
impl InitComplete {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    pub fn __repr__(&self, _py: Python) -> String {
        String::from("InitComplete()")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::InitComplete::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::InitCompleteRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::InitComplete::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
