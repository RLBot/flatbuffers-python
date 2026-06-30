use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct EnvironmentVariable {
    #[pyo3(set)]
    pub name: Py<PyString>,
    #[pyo3(set)]
    pub value: Py<PyString>,
}

impl crate::PyDefault for EnvironmentVariable {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                name: crate::pydefault_string(py),
                value: crate::pydefault_string(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::EnvironmentVariable> for EnvironmentVariable {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::EnvironmentVariable) -> Self {
        EnvironmentVariable {
            name: PyString::new(py, &flat_t.name).unbind(),
            value: PyString::new(py, &flat_t.value).unbind(),
        }
    }
}

impl FromGil<&EnvironmentVariable> for flat::EnvironmentVariable {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &EnvironmentVariable) -> Self {
        Self {
            name: py_type.name.to_str(py).unwrap().to_string(),
            value: py_type.value.to_str(py).unwrap().to_string(),
        }
    }
}

#[pymethods]
impl EnvironmentVariable {
    #[new]
    #[pyo3(signature = (name=None, value=None))]
    pub fn new(py: Python, name: Option<Py<PyString>>, value: Option<Py<PyString>>) -> Self {
        Self {
            name: name.unwrap_or_else(|| crate::pydefault_string(py)),
            value: value.unwrap_or_else(|| crate::pydefault_string(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "EnvironmentVariable(name={:?}, value={:?})",
            self.name.bind(py).to_cow().unwrap(),
            self.value.bind(py).to_cow().unwrap(),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("name", "value")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::EnvironmentVariable::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref =
            flat::EnvironmentVariableRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::EnvironmentVariable::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
