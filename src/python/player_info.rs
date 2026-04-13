use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct PlayerInfo {
    pub physics: Py<super::Physics>,
    pub score_info: Py<super::ScoreInfo>,
    pub hitbox: Py<super::BoxShape>,
    pub hitbox_offset: Py<super::Vector3>,
    pub latest_touch: Option<Py<super::Touch>>,
    pub air_state: super::AirState,
    pub dodge_timeout: Py<PyFloat>,
    pub demolished_timeout: Py<PyFloat>,
    pub is_supersonic: bool,
    pub is_bot: bool,
    pub name: Py<PyString>,
    pub team: u32,
    pub boost: Py<PyFloat>,
    pub player_id: i32,
    pub accolades: Py<PyList>,
    pub last_input: Py<super::ControllerState>,
    pub has_jumped: bool,
    pub has_double_jumped: bool,
    pub has_dodged: bool,
    pub dodge_elapsed: Py<PyFloat>,
    pub dodge_dir: Py<super::Vector2>,
}

impl crate::PyDefault for PlayerInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                physics: super::Physics::py_default(py),
                score_info: super::ScoreInfo::py_default(py),
                hitbox: super::BoxShape::py_default(py),
                hitbox_offset: super::Vector3::py_default(py),
                latest_touch: None,
                air_state: Default::default(),
                dodge_timeout: crate::pyfloat_default(py),
                demolished_timeout: crate::pyfloat_default(py),
                is_supersonic: Default::default(),
                is_bot: Default::default(),
                name: crate::pydefault_string(py),
                team: Default::default(),
                boost: crate::pyfloat_default(py),
                player_id: Default::default(),
                accolades: PyList::empty(py).unbind(),
                last_input: super::ControllerState::py_default(py),
                has_jumped: Default::default(),
                has_double_jumped: Default::default(),
                has_dodged: Default::default(),
                dodge_elapsed: crate::pyfloat_default(py),
                dodge_dir: super::Vector2::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PlayerInfo> for PlayerInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PlayerInfo) -> Self {
        PlayerInfo {
            physics: crate::into_py_from(py, &flat_t.physics),
            score_info: crate::into_py_from(py, &flat_t.score_info),
            hitbox: crate::into_py_from(py, &*flat_t.hitbox),
            hitbox_offset: crate::into_py_from(py, &flat_t.hitbox_offset),
            latest_touch: flat_t
                .latest_touch
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            air_state: flat_t.air_state,
            dodge_timeout: crate::float_to_py(py, flat_t.dodge_timeout),
            demolished_timeout: crate::float_to_py(py, flat_t.demolished_timeout),
            is_supersonic: flat_t.is_supersonic,
            is_bot: flat_t.is_bot,
            name: PyString::new(py, &flat_t.name).unbind(),
            team: flat_t.team,
            boost: crate::float_to_py(py, flat_t.boost),
            player_id: flat_t.player_id,
            accolades: crate::into_pystringlist_from(py, &flat_t.accolades),
            last_input: crate::into_py_from(py, &flat_t.last_input),
            has_jumped: flat_t.has_jumped,
            has_double_jumped: flat_t.has_double_jumped,
            has_dodged: flat_t.has_dodged,
            dodge_elapsed: crate::float_to_py(py, flat_t.dodge_elapsed),
            dodge_dir: crate::into_py_from(py, &flat_t.dodge_dir),
        }
    }
}

impl FromGil<&PlayerInfo> for flat::PlayerInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PlayerInfo) -> Self {
        Self {
            physics: crate::from_py_into(py, &py_type.physics),
            score_info: crate::from_py_into(py, &py_type.score_info),
            hitbox: Box::new(crate::from_py_into(py, &py_type.hitbox)),
            hitbox_offset: crate::from_py_into(py, &py_type.hitbox_offset),
            latest_touch: py_type
                .latest_touch
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            air_state: py_type.air_state,
            dodge_timeout: crate::float_from_py(py, &py_type.dodge_timeout),
            demolished_timeout: crate::float_from_py(py, &py_type.demolished_timeout),
            is_supersonic: py_type.is_supersonic,
            is_bot: py_type.is_bot,
            name: py_type.name.to_str(py).unwrap().to_string(),
            team: py_type.team,
            boost: crate::float_from_py(py, &py_type.boost),
            player_id: py_type.player_id,
            accolades: py_type
                .accolades
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pystring_into(x))
                .collect(),
            last_input: crate::from_py_into(py, &py_type.last_input),
            has_jumped: py_type.has_jumped,
            has_double_jumped: py_type.has_double_jumped,
            has_dodged: py_type.has_dodged,
            dodge_elapsed: crate::float_from_py(py, &py_type.dodge_elapsed),
            dodge_dir: crate::from_py_into(py, &py_type.dodge_dir),
        }
    }
}

#[pymethods]
impl PlayerInfo {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (physics=None, score_info=None, hitbox=None, hitbox_offset=None, latest_touch=None, air_state=Default::default(), dodge_timeout=0.0, demolished_timeout=0.0, is_supersonic=false, is_bot=false, name=None, team=0, boost=0.0, player_id=0, accolades=None, last_input=None, has_jumped=false, has_double_jumped=false, has_dodged=false, dodge_elapsed=0.0, dodge_dir=None))]
    pub fn new(
        py: Python,
        physics: Option<Py<super::Physics>>,
        score_info: Option<Py<super::ScoreInfo>>,
        hitbox: Option<Py<super::BoxShape>>,
        hitbox_offset: Option<Py<super::Vector3>>,
        latest_touch: Option<Py<super::Touch>>,
        air_state: super::AirState,
        dodge_timeout: f64,
        demolished_timeout: f64,
        is_supersonic: bool,
        is_bot: bool,
        name: Option<Py<PyString>>,
        team: u32,
        boost: f64,
        player_id: i32,
        accolades: Option<Py<PyList>>,
        last_input: Option<Py<super::ControllerState>>,
        has_jumped: bool,
        has_double_jumped: bool,
        has_dodged: bool,
        dodge_elapsed: f64,
        dodge_dir: Option<Py<super::Vector2>>,
    ) -> Self {
        Self {
            physics: physics.unwrap_or_else(|| super::Physics::py_default(py)),
            score_info: score_info.unwrap_or_else(|| super::ScoreInfo::py_default(py)),
            hitbox: hitbox.unwrap_or_else(|| super::BoxShape::py_default(py)),
            hitbox_offset: hitbox_offset.unwrap_or_else(|| super::Vector3::py_default(py)),
            latest_touch,
            air_state,
            dodge_timeout: PyFloat::new(py, dodge_timeout).unbind(),
            demolished_timeout: PyFloat::new(py, demolished_timeout).unbind(),
            is_supersonic,
            is_bot,
            name: name.unwrap_or_else(|| crate::pydefault_string(py)),
            team,
            boost: PyFloat::new(py, boost).unbind(),
            player_id,
            accolades: accolades.unwrap_or_else(|| PyList::empty(py).unbind()),
            last_input: last_input.unwrap_or_else(|| super::ControllerState::py_default(py)),
            has_jumped,
            has_double_jumped,
            has_dodged,
            dodge_elapsed: PyFloat::new(py, dodge_elapsed).unbind(),
            dodge_dir: dodge_dir.unwrap_or_else(|| super::Vector2::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "PlayerInfo(physics={}, score_info={}, hitbox={}, hitbox_offset={}, latest_touch={}, air_state={}, dodge_timeout={}, demolished_timeout={}, is_supersonic={}, is_bot={}, name={:?}, team={}, boost={}, player_id={}, accolades=[{}], last_input={}, has_jumped={}, has_double_jumped={}, has_dodged={}, dodge_elapsed={}, dodge_dir={})",
            self.physics.borrow(py).__repr__(py),
            self.score_info.borrow(py).__repr__(py),
            self.hitbox.borrow(py).__repr__(py),
            self.hitbox_offset.borrow(py).__repr__(py),
            self.latest_touch
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.air_state.__repr__(),
            self.dodge_timeout,
            self.demolished_timeout,
            crate::bool_to_str(self.is_supersonic),
            crate::bool_to_str(self.is_bot),
            self.name.bind(py).to_cow().unwrap(),
            self.team,
            self.boost,
            self.player_id,
            self.accolades
                .bind_borrowed(py)
                .iter()
                .map(|s| crate::format_string(crate::from_pystring_into(s)))
                .collect::<Vec<String>>()
                .join(", "),
            self.last_input.borrow(py).__repr__(py),
            crate::bool_to_str(self.has_jumped),
            crate::bool_to_str(self.has_double_jumped),
            crate::bool_to_str(self.has_dodged),
            self.dodge_elapsed,
            self.dodge_dir.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__(py: Python) -> Bound<pyo3::types::PyTuple> {
        pyo3::types::PyTuple::new(
            py,
            [
                "physics",
                "score_info",
                "hitbox",
                "hitbox_offset",
                "latest_touch",
                "air_state",
                "dodge_timeout",
                "demolished_timeout",
                "is_supersonic",
                "is_bot",
                "name",
                "team",
                "boost",
                "player_id",
                "accolades",
                "last_input",
                "has_jumped",
                "has_double_jumped",
                "has_dodged",
                "dodge_elapsed",
                "dodge_dir",
            ],
        )
        .unwrap()
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PlayerInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::PlayerInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PlayerInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
