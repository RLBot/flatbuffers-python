use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct PingResponse {
    #[pyo3(set)]
    pub cookie: u64,
}

impl crate::PyDefault for PingResponse {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                cookie: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PingResponse> for PingResponse {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PingResponse) -> Self {
        PingResponse {
            cookie: flat_t.cookie,
        }
    }
}

impl FromGil<&PingResponse> for flat::PingResponse {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PingResponse) -> Self {
        Self {
            cookie: py_type.cookie,
        }
    }
}

#[pymethods]
impl PingResponse {
    #[new]
    #[pyo3(signature = (cookie=0))]
    pub fn new(cookie: u64) -> Self {
        Self { cookie }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!("PingResponse(cookie={})", self.cookie,)
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("cookie",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PingResponse::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::PingResponseRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PingResponse::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
