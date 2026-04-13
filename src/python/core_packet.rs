use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct CorePacket {
    pub message: Py<PyAny>,
}

impl crate::PyDefault for CorePacket {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                message: super::CoreMessage::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::CorePacket> for CorePacket {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::CorePacket) -> Self {
        CorePacket {
            message: IntoGil::<super::CoreMessage>::into_gil(&flat_t.message, py).into_any(),
        }
    }
}

impl FromGil<&CorePacket> for flat::CorePacket {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &CorePacket) -> Self {
        Self {
            message: super::CoreMessage::extract(py_type.message.bind_borrowed(py))
                .as_ref()
                .unwrap()
                .into_gil(py),
        }
    }
}

#[pymethods]
impl CorePacket {
    #[new]
    #[pyo3(signature = (message=None))]
    pub fn new(py: Python, message: Option<Py<PyAny>>) -> Self {
        Self {
            message: message.unwrap_or_else(|| super::CoreMessage::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "CorePacket(message={})",
            super::CoreMessage::extract(self.message.bind_borrowed(py))
                .unwrap()
                .__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("message",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::CorePacket::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::CorePacketRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::CorePacket::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
