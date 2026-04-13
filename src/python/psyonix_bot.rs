use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct PsyonixBot {
    #[pyo3(set)]
    pub name: Py<PyString>,
    #[pyo3(set)]
    pub loadout: Option<Py<super::PlayerLoadout>>,
    #[pyo3(set)]
    pub bot_skill: super::PsyonixSkill,
}

impl crate::PyDefault for PsyonixBot {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                name: crate::pydefault_string(py),
                loadout: None,
                bot_skill: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PsyonixBot> for PsyonixBot {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PsyonixBot) -> Self {
        PsyonixBot {
            name: PyString::new(py, &flat_t.name).unbind(),
            loadout: flat_t
                .loadout
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            bot_skill: flat_t.bot_skill,
        }
    }
}

impl FromGil<&PsyonixBot> for flat::PsyonixBot {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PsyonixBot) -> Self {
        Self {
            name: py_type.name.to_str(py).unwrap().to_string(),
            loadout: py_type
                .loadout
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            bot_skill: py_type.bot_skill,
        }
    }
}

#[pymethods]
impl PsyonixBot {
    #[new]
    #[pyo3(signature = (name=None, loadout=None, bot_skill=Default::default()))]
    pub fn new(
        py: Python,
        name: Option<Py<PyString>>,
        loadout: Option<Py<super::PlayerLoadout>>,
        bot_skill: super::PsyonixSkill,
    ) -> Self {
        Self {
            name: name.unwrap_or_else(|| crate::pydefault_string(py)),
            loadout,
            bot_skill,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "PsyonixBot(name={:?}, loadout={}, bot_skill={})",
            self.name.bind(py).to_cow().unwrap(),
            self.loadout
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.bot_skill.__repr__(),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("name", "loadout", "bot_skill")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PsyonixBot::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::PsyonixBotRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PsyonixBot::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
