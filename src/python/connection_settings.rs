use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct ConnectionSettings {
    #[pyo3(set)]
    pub agent_id: Py<PyString>,
    #[pyo3(set)]
    pub wants_ball_predictions: bool,
    #[pyo3(set)]
    pub wants_comms: bool,
    #[pyo3(set)]
    pub close_between_matches: bool,
}

impl crate::PyDefault for ConnectionSettings {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                agent_id: crate::pydefault_string(py),
                wants_ball_predictions: Default::default(),
                wants_comms: Default::default(),
                close_between_matches: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::ConnectionSettings> for ConnectionSettings {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::ConnectionSettings) -> Self {
        ConnectionSettings {
            agent_id: PyString::new(py, &flat_t.agent_id).unbind(),
            wants_ball_predictions: flat_t.wants_ball_predictions,
            wants_comms: flat_t.wants_comms,
            close_between_matches: flat_t.close_between_matches,
        }
    }
}

impl FromGil<&ConnectionSettings> for flat::ConnectionSettings {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &ConnectionSettings) -> Self {
        Self {
            agent_id: py_type.agent_id.to_str(py).unwrap().to_string(),
            wants_ball_predictions: py_type.wants_ball_predictions,
            wants_comms: py_type.wants_comms,
            close_between_matches: py_type.close_between_matches,
        }
    }
}

#[pymethods]
impl ConnectionSettings {
    #[new]
    #[pyo3(signature = (agent_id=None, wants_ball_predictions=false, wants_comms=false, close_between_matches=false))]
    pub fn new(
        py: Python,
        agent_id: Option<Py<PyString>>,
        wants_ball_predictions: bool,
        wants_comms: bool,
        close_between_matches: bool,
    ) -> Self {
        Self {
            agent_id: agent_id.unwrap_or_else(|| crate::pydefault_string(py)),
            wants_ball_predictions,
            wants_comms,
            close_between_matches,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "ConnectionSettings(agent_id={:?}, wants_ball_predictions={}, wants_comms={}, close_between_matches={})",
            self.agent_id.bind(py).to_cow().unwrap(),
            crate::bool_to_str(self.wants_ball_predictions),
            crate::bool_to_str(self.wants_comms),
            crate::bool_to_str(self.close_between_matches),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        (
            "agent_id",
            "wants_ball_predictions",
            "wants_comms",
            "close_between_matches",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::ConnectionSettings::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::ConnectionSettingsRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::ConnectionSettings::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
