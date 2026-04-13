use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct CustomBot {
    #[pyo3(set)]
    pub name: Py<PyString>,
    #[pyo3(set)]
    pub root_dir: Py<PyString>,
    #[pyo3(set)]
    pub run_command: Py<PyString>,
    #[pyo3(set)]
    pub loadout: Option<Py<super::PlayerLoadout>>,
    #[pyo3(set)]
    pub agent_id: Py<PyString>,
    #[pyo3(set)]
    pub hivemind: bool,
}

impl crate::PyDefault for CustomBot {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                name: crate::pydefault_string(py),
                root_dir: crate::pydefault_string(py),
                run_command: crate::pydefault_string(py),
                loadout: None,
                agent_id: crate::pydefault_string(py),
                hivemind: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::CustomBot> for CustomBot {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::CustomBot) -> Self {
        CustomBot {
            name: PyString::new(py, &flat_t.name).unbind(),
            root_dir: PyString::new(py, &flat_t.root_dir).unbind(),
            run_command: PyString::new(py, &flat_t.run_command).unbind(),
            loadout: flat_t
                .loadout
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            agent_id: PyString::new(py, &flat_t.agent_id).unbind(),
            hivemind: flat_t.hivemind,
        }
    }
}

impl FromGil<&CustomBot> for flat::CustomBot {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &CustomBot) -> Self {
        Self {
            name: py_type.name.to_str(py).unwrap().to_string(),
            root_dir: py_type.root_dir.to_str(py).unwrap().to_string(),
            run_command: py_type.run_command.to_str(py).unwrap().to_string(),
            loadout: py_type
                .loadout
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            agent_id: py_type.agent_id.to_str(py).unwrap().to_string(),
            hivemind: py_type.hivemind,
        }
    }
}

#[pymethods]
impl CustomBot {
    #[new]
    #[pyo3(signature = (name=None, root_dir=None, run_command=None, loadout=None, agent_id=None, hivemind=false))]
    pub fn new(
        py: Python,
        name: Option<Py<PyString>>,
        root_dir: Option<Py<PyString>>,
        run_command: Option<Py<PyString>>,
        loadout: Option<Py<super::PlayerLoadout>>,
        agent_id: Option<Py<PyString>>,
        hivemind: bool,
    ) -> Self {
        Self {
            name: name.unwrap_or_else(|| crate::pydefault_string(py)),
            root_dir: root_dir.unwrap_or_else(|| crate::pydefault_string(py)),
            run_command: run_command.unwrap_or_else(|| crate::pydefault_string(py)),
            loadout,
            agent_id: agent_id.unwrap_or_else(|| crate::pydefault_string(py)),
            hivemind,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "CustomBot(name={:?}, root_dir={:?}, run_command={:?}, loadout={}, agent_id={:?}, hivemind={})",
            self.name.bind(py).to_cow().unwrap(),
            self.root_dir.bind(py).to_cow().unwrap(),
            self.run_command.bind(py).to_cow().unwrap(),
            self.loadout
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.agent_id.bind(py).to_cow().unwrap(),
            crate::bool_to_str(self.hivemind),
        )
    }

    #[classattr]
    fn __match_args__() -> (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ) {
        (
            "name",
            "root_dir",
            "run_command",
            "loadout",
            "agent_id",
            "hivemind",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::CustomBot::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::CustomBotRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::CustomBot::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
