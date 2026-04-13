use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct DesiredBallState {
    #[pyo3(set)]
    pub physics: Py<super::DesiredPhysics>,
}

impl crate::PyDefault for DesiredBallState {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                physics: super::DesiredPhysics::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::DesiredBallState> for DesiredBallState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::DesiredBallState) -> Self {
        DesiredBallState {
            physics: crate::into_py_from(py, &*flat_t.physics),
        }
    }
}

impl FromGil<&DesiredBallState> for flat::DesiredBallState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &DesiredBallState) -> Self {
        Self {
            physics: Box::new(crate::from_py_into(py, &py_type.physics)),
        }
    }
}

#[pymethods]
impl DesiredBallState {
    #[new]
    #[pyo3(signature = (physics=None))]
    pub fn new(py: Python, physics: Option<Py<super::DesiredPhysics>>) -> Self {
        Self {
            physics: physics.unwrap_or_else(|| super::DesiredPhysics::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "DesiredBallState(physics={})",
            self.physics.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("physics",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::DesiredBallState::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::DesiredBallStateRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::DesiredBallState::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
