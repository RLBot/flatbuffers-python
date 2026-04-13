use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct StartCommand {
    #[pyo3(set)]
    pub config_path: Py<PyString>,
}

impl crate::PyDefault for StartCommand {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                config_path: crate::pydefault_string(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::StartCommand> for StartCommand {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::StartCommand) -> Self {
        StartCommand {
            config_path: PyString::new(py, &flat_t.config_path).unbind(),
        }
    }
}

impl FromGil<&StartCommand> for flat::StartCommand {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &StartCommand) -> Self {
        Self {
            config_path: py_type.config_path.to_str(py).unwrap().to_string(),
        }
    }
}

#[pymethods]
impl StartCommand {
    #[new]
    #[pyo3(signature = (config_path=None))]
    pub fn new(py: Python, config_path: Option<Py<PyString>>) -> Self {
        Self {
            config_path: config_path.unwrap_or_else(|| crate::pydefault_string(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "StartCommand(config_path={:?})",
            self.config_path.bind(py).to_cow().unwrap(),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("config_path",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::StartCommand::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::StartCommandRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::StartCommand::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
