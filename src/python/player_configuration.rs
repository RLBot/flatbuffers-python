use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct PlayerConfiguration {
    #[pyo3(set)]
    pub variety: Py<PyAny>,
    #[pyo3(set)]
    pub team: u32,
    #[pyo3(set)]
    pub player_id: i32,
}

impl crate::PyDefault for PlayerConfiguration {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                variety: super::PlayerClass::py_default(py),
                team: Default::default(),
                player_id: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PlayerConfiguration> for PlayerConfiguration {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PlayerConfiguration) -> Self {
        PlayerConfiguration {
            variety: IntoGil::<super::PlayerClass>::into_gil(&flat_t.variety, py).into_any(),
            team: flat_t.team,
            player_id: flat_t.player_id,
        }
    }
}

impl FromGil<&PlayerConfiguration> for flat::PlayerConfiguration {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PlayerConfiguration) -> Self {
        Self {
            variety: super::PlayerClass::extract(py_type.variety.bind_borrowed(py))
                .as_ref()
                .unwrap()
                .into_gil(py),
            team: py_type.team,
            player_id: py_type.player_id,
        }
    }
}

#[pymethods]
impl PlayerConfiguration {
    #[new]
    #[pyo3(signature = (variety=None, team=0, player_id=0))]
    pub fn new(py: Python, variety: Option<Py<PyAny>>, team: u32, player_id: i32) -> Self {
        Self {
            variety: variety.unwrap_or_else(|| super::PlayerClass::py_default(py)),
            team,
            player_id,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "PlayerConfiguration(variety={}, team={}, player_id={})",
            super::PlayerClass::extract(self.variety.bind_borrowed(py))
                .unwrap()
                .__repr__(py),
            self.team,
            self.player_id,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("variety", "team", "player_id")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PlayerConfiguration::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref =
            flat::PlayerConfigurationRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PlayerConfiguration::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
