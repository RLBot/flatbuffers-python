use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct ControllableTeamInfo {
    pub team: u32,
    pub controllables: Py<PyList>,
}

impl crate::PyDefault for ControllableTeamInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                team: Default::default(),
                controllables: PyList::empty(py).unbind(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::ControllableTeamInfo> for ControllableTeamInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::ControllableTeamInfo) -> Self {
        ControllableTeamInfo {
            team: flat_t.team,
            controllables: PyList::new(
                py,
                flat_t
                    .controllables
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::ControllableInfo>(py, x)),
            )
            .unwrap()
            .unbind(),
        }
    }
}

impl FromGil<&ControllableTeamInfo> for flat::ControllableTeamInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &ControllableTeamInfo) -> Self {
        Self {
            team: py_type.team,
            controllables: py_type
                .controllables
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
        }
    }
}

#[pymethods]
impl ControllableTeamInfo {
    #[new]
    #[pyo3(signature = (team=0, controllables=None))]
    pub fn new(py: Python, team: u32, controllables: Option<Py<PyList>>) -> Self {
        Self {
            team,
            controllables: controllables.unwrap_or_else(|| PyList::empty(py).unbind()),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "ControllableTeamInfo(team={}, controllables=[{}])",
            self.team,
            self.controllables
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::ControllableInfo>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("team", "controllables")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::ControllableTeamInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref =
            flat::ControllableTeamInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::ControllableTeamInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
