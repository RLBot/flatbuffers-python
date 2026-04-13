use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct BoostPad {
    pub location: Py<super::Vector3>,
    pub is_full_boost: bool,
}

impl crate::PyDefault for BoostPad {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                location: super::Vector3::py_default(py),
                is_full_boost: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::BoostPad> for BoostPad {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::BoostPad) -> Self {
        BoostPad {
            location: crate::into_py_from(py, &flat_t.location),
            is_full_boost: flat_t.is_full_boost,
        }
    }
}

impl FromGil<&BoostPad> for flat::BoostPad {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &BoostPad) -> Self {
        Self {
            location: crate::from_py_into(py, &py_type.location),
            is_full_boost: py_type.is_full_boost,
        }
    }
}

#[pymethods]
impl BoostPad {
    #[new]
    #[pyo3(signature = (location=None, is_full_boost=false))]
    pub fn new(py: Python, location: Option<Py<super::Vector3>>, is_full_boost: bool) -> Self {
        Self {
            location: location.unwrap_or_else(|| super::Vector3::py_default(py)),
            is_full_boost,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "BoostPad(location={}, is_full_boost={})",
            self.location.borrow(py).__repr__(py),
            crate::bool_to_str(self.is_full_boost),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("location", "is_full_boost")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::BoostPad::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::BoostPadRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::BoostPad::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
