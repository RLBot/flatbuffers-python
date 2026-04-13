use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct ControllableInfo {
    pub index: u32,
    pub identifier: i32,
}

impl crate::PyDefault for ControllableInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                index: Default::default(),
                identifier: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::ControllableInfo> for ControllableInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::ControllableInfo) -> Self {
        ControllableInfo {
            index: flat_t.index,
            identifier: flat_t.identifier,
        }
    }
}

impl FromGil<&ControllableInfo> for flat::ControllableInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &ControllableInfo) -> Self {
        Self {
            index: py_type.index,
            identifier: py_type.identifier,
        }
    }
}

#[pymethods]
impl ControllableInfo {
    #[new]
    #[pyo3(signature = (index=0, identifier=0))]
    pub fn new(index: u32, identifier: i32) -> Self {
        Self { index, identifier }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "ControllableInfo(index={}, identifier={})",
            self.index, self.identifier,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("index", "identifier")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::ControllableInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::ControllableInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::ControllableInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
