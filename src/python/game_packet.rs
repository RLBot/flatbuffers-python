use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct GamePacket {
    pub players: Py<PyList>,
    pub boost_pads: Py<PyList>,
    pub balls: Py<PyList>,
    pub match_info: Py<super::MatchInfo>,
    pub teams: Py<PyList>,
}

impl crate::PyDefault for GamePacket {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                players: PyList::empty(py).unbind(),
                boost_pads: PyList::empty(py).unbind(),
                balls: PyList::empty(py).unbind(),
                match_info: super::MatchInfo::py_default(py),
                teams: PyList::empty(py).unbind(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::GamePacket> for GamePacket {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::GamePacket) -> Self {
        GamePacket {
            players: PyList::new(
                py,
                flat_t
                    .players
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::PlayerInfo>(py, x)),
            )
            .unwrap()
            .unbind(),
            boost_pads: PyList::new(
                py,
                flat_t
                    .boost_pads
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::BoostPadState>(py, x)),
            )
            .unwrap()
            .unbind(),
            balls: PyList::new(
                py,
                flat_t
                    .balls
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::BallInfo>(py, x)),
            )
            .unwrap()
            .unbind(),
            match_info: crate::into_py_from(py, &*flat_t.match_info),
            teams: PyList::new(
                py,
                flat_t
                    .teams
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::TeamInfo>(py, x)),
            )
            .unwrap()
            .unbind(),
        }
    }
}

impl FromGil<&GamePacket> for flat::GamePacket {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &GamePacket) -> Self {
        Self {
            players: py_type
                .players
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            boost_pads: py_type
                .boost_pads
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            balls: py_type
                .balls
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            match_info: Box::new(crate::from_py_into(py, &py_type.match_info)),
            teams: py_type
                .teams
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
        }
    }
}

#[pymethods]
impl GamePacket {
    #[new]
    #[pyo3(signature = (players=None, boost_pads=None, balls=None, match_info=None, teams=None))]
    pub fn new(
        py: Python,
        players: Option<Py<PyList>>,
        boost_pads: Option<Py<PyList>>,
        balls: Option<Py<PyList>>,
        match_info: Option<Py<super::MatchInfo>>,
        teams: Option<Py<PyList>>,
    ) -> Self {
        Self {
            players: players.unwrap_or_else(|| PyList::empty(py).unbind()),
            boost_pads: boost_pads.unwrap_or_else(|| PyList::empty(py).unbind()),
            balls: balls.unwrap_or_else(|| PyList::empty(py).unbind()),
            match_info: match_info.unwrap_or_else(|| super::MatchInfo::py_default(py)),
            teams: teams.unwrap_or_else(|| PyList::empty(py).unbind()),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "GamePacket(players=[{}], boost_pads=[{}], balls=[{}], match_info={}, teams=[{}])",
            self.players
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::PlayerInfo>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.boost_pads
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::BoostPadState>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.balls
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::BallInfo>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.match_info.borrow(py).__repr__(py),
            self.teams
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::TeamInfo>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
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
        ("players", "boost_pads", "balls", "match_info", "teams")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::GamePacket::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::GamePacketRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::GamePacket::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
