use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct ConsoleCommand {
    #[pyo3(set)]
    pub command: Py<PyString>,
}

impl crate::PyDefault for ConsoleCommand {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                command: crate::pydefault_string(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::ConsoleCommand> for ConsoleCommand {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::ConsoleCommand) -> Self {
        ConsoleCommand {
            command: PyString::new(py, &flat_t.command).unbind(),
        }
    }
}

impl FromGil<&ConsoleCommand> for flat::ConsoleCommand {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &ConsoleCommand) -> Self {
        Self {
            command: py_type.command.to_str(py).unwrap().to_string(),
        }
    }
}

#[pymethods]
impl ConsoleCommand {
    #[new]
    #[pyo3(signature = (command=None))]
    pub fn new(py: Python, command: Option<Py<PyString>>) -> Self {
        Self {
            command: command.unwrap_or_else(|| crate::pydefault_string(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "ConsoleCommand(command={:?})",
            self.command.bind(py).to_cow().unwrap(),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("command",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::ConsoleCommand::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::ConsoleCommandRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::ConsoleCommand::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
