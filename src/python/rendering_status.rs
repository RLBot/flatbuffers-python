use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct RenderingStatus {
    #[pyo3(set)]
    pub index: u32,
    #[pyo3(set)]
    pub is_bot: bool,
    #[pyo3(set)]
    pub status: bool,
}

impl crate::PyDefault for RenderingStatus {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                index: Default::default(),
                is_bot: Default::default(),
                status: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::RenderingStatus> for RenderingStatus {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::RenderingStatus) -> Self {
        RenderingStatus {
            index: flat_t.index,
            is_bot: flat_t.is_bot,
            status: flat_t.status,
        }
    }
}

impl FromGil<&RenderingStatus> for flat::RenderingStatus {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &RenderingStatus) -> Self {
        Self {
            index: py_type.index,
            is_bot: py_type.is_bot,
            status: py_type.status,
        }
    }
}

#[pymethods]
impl RenderingStatus {
    #[new]
    #[pyo3(signature = (index=0, is_bot=false, status=false))]
    pub fn new(index: u32, is_bot: bool, status: bool) -> Self {
        Self {
            index,
            is_bot,
            status,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "RenderingStatus(index={}, is_bot={}, status={})",
            self.index,
            crate::bool_to_str(self.is_bot),
            crate::bool_to_str(self.status),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("index", "is_bot", "status")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::RenderingStatus::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::RenderingStatusRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::RenderingStatus::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
