use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct ScriptConfiguration {
    #[pyo3(set)]
    pub name: Py<PyString>,
    #[pyo3(set)]
    pub root_dir: Py<PyString>,
    #[pyo3(set)]
    pub run_command: Py<PyString>,
    #[pyo3(set)]
    pub script_id: i32,
    #[pyo3(set)]
    pub agent_id: Py<PyString>,
}

impl crate::PyDefault for ScriptConfiguration {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                name: crate::pydefault_string(py),
                root_dir: crate::pydefault_string(py),
                run_command: crate::pydefault_string(py),
                script_id: Default::default(),
                agent_id: crate::pydefault_string(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::ScriptConfiguration> for ScriptConfiguration {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::ScriptConfiguration) -> Self {
        ScriptConfiguration {
            name: PyString::new(py, &flat_t.name).unbind(),
            root_dir: PyString::new(py, &flat_t.root_dir).unbind(),
            run_command: PyString::new(py, &flat_t.run_command).unbind(),
            script_id: flat_t.script_id,
            agent_id: PyString::new(py, &flat_t.agent_id).unbind(),
        }
    }
}

impl FromGil<&ScriptConfiguration> for flat::ScriptConfiguration {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &ScriptConfiguration) -> Self {
        Self {
            name: py_type.name.to_str(py).unwrap().to_string(),
            root_dir: py_type.root_dir.to_str(py).unwrap().to_string(),
            run_command: py_type.run_command.to_str(py).unwrap().to_string(),
            script_id: py_type.script_id,
            agent_id: py_type.agent_id.to_str(py).unwrap().to_string(),
        }
    }
}

#[pymethods]
impl ScriptConfiguration {
    #[new]
    #[pyo3(signature = (name=None, root_dir=None, run_command=None, script_id=0, agent_id=None))]
    pub fn new(
        py: Python,
        name: Option<Py<PyString>>,
        root_dir: Option<Py<PyString>>,
        run_command: Option<Py<PyString>>,
        script_id: i32,
        agent_id: Option<Py<PyString>>,
    ) -> Self {
        Self {
            name: name.unwrap_or_else(|| crate::pydefault_string(py)),
            root_dir: root_dir.unwrap_or_else(|| crate::pydefault_string(py)),
            run_command: run_command.unwrap_or_else(|| crate::pydefault_string(py)),
            script_id,
            agent_id: agent_id.unwrap_or_else(|| crate::pydefault_string(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "ScriptConfiguration(name={:?}, root_dir={:?}, run_command={:?}, script_id={}, agent_id={:?})",
            self.name.bind(py).to_cow().unwrap(),
            self.root_dir.bind(py).to_cow().unwrap(),
            self.run_command.bind(py).to_cow().unwrap(),
            self.script_id,
            self.agent_id.bind(py).to_cow().unwrap(),
        )
    }

    #[classattr]
    fn __match_args__() -> (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ) {
        ("name", "root_dir", "run_command", "script_id", "agent_id")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::ScriptConfiguration::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref =
            flat::ScriptConfigurationRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::ScriptConfiguration::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
