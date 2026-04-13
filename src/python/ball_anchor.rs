use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct BallAnchor {
    #[pyo3(set)]
    pub index: u32,
    #[pyo3(set)]
    pub local: Py<super::Vector3>,
}

impl crate::PyDefault for BallAnchor {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                index: Default::default(),
                local: super::Vector3::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::BallAnchor> for BallAnchor {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::BallAnchor) -> Self {
        BallAnchor {
            index: flat_t.index,
            local: crate::into_py_from(py, &flat_t.local),
        }
    }
}

impl FromGil<&BallAnchor> for flat::BallAnchor {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &BallAnchor) -> Self {
        Self {
            index: py_type.index,
            local: crate::from_py_into(py, &py_type.local),
        }
    }
}

#[pymethods]
impl BallAnchor {
    #[new]
    #[pyo3(signature = (index=0, local=None))]
    pub fn new(py: Python, index: u32, local: Option<Py<super::Vector3>>) -> Self {
        Self {
            index,
            local: local.unwrap_or_else(|| super::Vector3::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "BallAnchor(index={}, local={})",
            self.index,
            self.local.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("index", "local")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::BallAnchor::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::BallAnchorRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::BallAnchor::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
