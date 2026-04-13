use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct MatchComm {
    #[pyo3(set)]
    pub index: u32,
    #[pyo3(set)]
    pub team: u32,
    #[pyo3(set)]
    pub team_only: bool,
    #[pyo3(set)]
    pub display: Option<Py<PyString>>,
    #[pyo3(set)]
    pub content: Py<PyBytes>,
}

impl crate::PyDefault for MatchComm {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                index: Default::default(),
                team: Default::default(),
                team_only: Default::default(),
                display: None,
                content: PyBytes::new(py, &[]).unbind(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::MatchComm> for MatchComm {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::MatchComm) -> Self {
        MatchComm {
            index: flat_t.index,
            team: flat_t.team,
            team_only: flat_t.team_only,
            display: flat_t
                .display
                .as_ref()
                .map(|s| PyString::new(py, s).unbind()),
            content: PyBytes::new(py, &flat_t.content).unbind(),
        }
    }
}

impl FromGil<&MatchComm> for flat::MatchComm {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &MatchComm) -> Self {
        Self {
            index: py_type.index,
            team: py_type.team,
            team_only: py_type.team_only,
            display: py_type
                .display
                .as_ref()
                .map(|s| s.to_str(py).unwrap().to_string()),
            content: py_type.content.as_bytes(py).to_vec(),
        }
    }
}

#[pymethods]
impl MatchComm {
    #[new]
    #[pyo3(signature = (index=0, team=0, team_only=false, display=None, content=None))]
    pub fn new(
        py: Python,
        index: u32,
        team: u32,
        team_only: bool,
        display: Option<Py<PyString>>,
        content: Option<Py<PyBytes>>,
    ) -> Self {
        Self {
            index,
            team,
            team_only,
            display,
            content: content.unwrap_or_else(|| PyBytes::new(py, &[]).unbind()),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "MatchComm(index={}, team={}, team_only={}, display={}, content=bytes([{}]))",
            self.index,
            self.team,
            crate::bool_to_str(self.team_only),
            self.display.as_ref().map_or_else(crate::none_str, |i| {
                crate::format_string(i.to_str(py).unwrap().to_string())
            }),
            self.content
                .as_bytes(py)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", "),
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
        ("index", "team", "team_only", "display", "content")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::MatchComm::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::MatchCommRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::MatchComm::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
