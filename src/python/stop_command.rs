use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct StopCommand {
    #[pyo3(set)]
    pub shutdown_server: bool,
}

impl crate::PyDefault for StopCommand {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                shutdown_server: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::StopCommand> for StopCommand {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::StopCommand) -> Self {
        StopCommand {
            shutdown_server: flat_t.shutdown_server,
        }
    }
}

impl FromGil<&StopCommand> for flat::StopCommand {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &StopCommand) -> Self {
        Self {
            shutdown_server: py_type.shutdown_server,
        }
    }
}

#[pymethods]
impl StopCommand {
    #[new]
    #[pyo3(signature = (shutdown_server=false))]
    pub fn new(shutdown_server: bool) -> Self {
        Self { shutdown_server }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "StopCommand(shutdown_server={})",
            crate::bool_to_str(self.shutdown_server),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("shutdown_server",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::StopCommand::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::StopCommandRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::StopCommand::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
