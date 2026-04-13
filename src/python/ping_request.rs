use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct PingRequest {
    #[pyo3(set)]
    pub cookie: u64,
}

impl crate::PyDefault for PingRequest {
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

impl FromGil<&flat::PingRequest> for PingRequest {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PingRequest) -> Self {
        PingRequest {
            cookie: flat_t.cookie,
        }
    }
}

impl FromGil<&PingRequest> for flat::PingRequest {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PingRequest) -> Self {
        Self {
            cookie: py_type.cookie,
        }
    }
}

#[pymethods]
impl PingRequest {
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
        format!("PingRequest(cookie={})", self.cookie,)
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("cookie",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PingRequest::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::PingRequestRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PingRequest::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
