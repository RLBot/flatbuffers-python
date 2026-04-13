use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct PlayerLoadout {
    #[pyo3(set)]
    pub team_color_id: u32,
    #[pyo3(set)]
    pub custom_color_id: u32,
    #[pyo3(set)]
    pub car_id: u32,
    #[pyo3(set)]
    pub decal_id: u32,
    #[pyo3(set)]
    pub wheels_id: u32,
    #[pyo3(set)]
    pub boost_id: u32,
    #[pyo3(set)]
    pub antenna_id: u32,
    #[pyo3(set)]
    pub hat_id: u32,
    #[pyo3(set)]
    pub paint_finish_id: u32,
    #[pyo3(set)]
    pub custom_finish_id: u32,
    #[pyo3(set)]
    pub engine_audio_id: u32,
    #[pyo3(set)]
    pub trails_id: u32,
    #[pyo3(set)]
    pub goal_explosion_id: u32,
    #[pyo3(set)]
    pub loadout_paint: Option<Py<super::LoadoutPaint>>,
    #[pyo3(set)]
    pub primary_color_lookup: Option<Py<super::Color>>,
    #[pyo3(set)]
    pub secondary_color_lookup: Option<Py<super::Color>>,
}

impl crate::PyDefault for PlayerLoadout {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                team_color_id: Default::default(),
                custom_color_id: Default::default(),
                car_id: Default::default(),
                decal_id: Default::default(),
                wheels_id: Default::default(),
                boost_id: Default::default(),
                antenna_id: Default::default(),
                hat_id: Default::default(),
                paint_finish_id: Default::default(),
                custom_finish_id: Default::default(),
                engine_audio_id: Default::default(),
                trails_id: Default::default(),
                goal_explosion_id: Default::default(),
                loadout_paint: None,
                primary_color_lookup: None,
                secondary_color_lookup: None,
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PlayerLoadout> for PlayerLoadout {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PlayerLoadout) -> Self {
        PlayerLoadout {
            team_color_id: flat_t.team_color_id,
            custom_color_id: flat_t.custom_color_id,
            car_id: flat_t.car_id,
            decal_id: flat_t.decal_id,
            wheels_id: flat_t.wheels_id,
            boost_id: flat_t.boost_id,
            antenna_id: flat_t.antenna_id,
            hat_id: flat_t.hat_id,
            paint_finish_id: flat_t.paint_finish_id,
            custom_finish_id: flat_t.custom_finish_id,
            engine_audio_id: flat_t.engine_audio_id,
            trails_id: flat_t.trails_id,
            goal_explosion_id: flat_t.goal_explosion_id,
            loadout_paint: flat_t
                .loadout_paint
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            primary_color_lookup: flat_t
                .primary_color_lookup
                .map(|x| crate::into_py_from(py, &x)),
            secondary_color_lookup: flat_t
                .secondary_color_lookup
                .map(|x| crate::into_py_from(py, &x)),
        }
    }
}

impl FromGil<&PlayerLoadout> for flat::PlayerLoadout {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PlayerLoadout) -> Self {
        Self {
            team_color_id: py_type.team_color_id,
            custom_color_id: py_type.custom_color_id,
            car_id: py_type.car_id,
            decal_id: py_type.decal_id,
            wheels_id: py_type.wheels_id,
            boost_id: py_type.boost_id,
            antenna_id: py_type.antenna_id,
            hat_id: py_type.hat_id,
            paint_finish_id: py_type.paint_finish_id,
            custom_finish_id: py_type.custom_finish_id,
            engine_audio_id: py_type.engine_audio_id,
            trails_id: py_type.trails_id,
            goal_explosion_id: py_type.goal_explosion_id,
            loadout_paint: py_type
                .loadout_paint
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            primary_color_lookup: py_type
                .primary_color_lookup
                .as_ref()
                .map(|x| crate::from_py_into(py, x)),
            secondary_color_lookup: py_type
                .secondary_color_lookup
                .as_ref()
                .map(|x| crate::from_py_into(py, x)),
        }
    }
}

#[pymethods]
impl PlayerLoadout {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (team_color_id=0, custom_color_id=0, car_id=0, decal_id=0, wheels_id=0, boost_id=0, antenna_id=0, hat_id=0, paint_finish_id=0, custom_finish_id=0, engine_audio_id=0, trails_id=0, goal_explosion_id=0, loadout_paint=None, primary_color_lookup=None, secondary_color_lookup=None))]
    pub fn new(
        team_color_id: u32,
        custom_color_id: u32,
        car_id: u32,
        decal_id: u32,
        wheels_id: u32,
        boost_id: u32,
        antenna_id: u32,
        hat_id: u32,
        paint_finish_id: u32,
        custom_finish_id: u32,
        engine_audio_id: u32,
        trails_id: u32,
        goal_explosion_id: u32,
        loadout_paint: Option<Py<super::LoadoutPaint>>,
        primary_color_lookup: Option<Py<super::Color>>,
        secondary_color_lookup: Option<Py<super::Color>>,
    ) -> Self {
        Self {
            team_color_id,
            custom_color_id,
            car_id,
            decal_id,
            wheels_id,
            boost_id,
            antenna_id,
            hat_id,
            paint_finish_id,
            custom_finish_id,
            engine_audio_id,
            trails_id,
            goal_explosion_id,
            loadout_paint,
            primary_color_lookup,
            secondary_color_lookup,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "PlayerLoadout(team_color_id={}, custom_color_id={}, car_id={}, decal_id={}, wheels_id={}, boost_id={}, antenna_id={}, hat_id={}, paint_finish_id={}, custom_finish_id={}, engine_audio_id={}, trails_id={}, goal_explosion_id={}, loadout_paint={}, primary_color_lookup={}, secondary_color_lookup={})",
            self.team_color_id,
            self.custom_color_id,
            self.car_id,
            self.decal_id,
            self.wheels_id,
            self.boost_id,
            self.antenna_id,
            self.hat_id,
            self.paint_finish_id,
            self.custom_finish_id,
            self.engine_audio_id,
            self.trails_id,
            self.goal_explosion_id,
            self.loadout_paint
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.primary_color_lookup
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.secondary_color_lookup
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
        )
    }

    #[classattr]
    fn __match_args__(py: Python) -> Bound<pyo3::types::PyTuple> {
        pyo3::types::PyTuple::new(
            py,
            [
                "team_color_id",
                "custom_color_id",
                "car_id",
                "decal_id",
                "wheels_id",
                "boost_id",
                "antenna_id",
                "hat_id",
                "paint_finish_id",
                "custom_finish_id",
                "engine_audio_id",
                "trails_id",
                "goal_explosion_id",
                "loadout_paint",
                "primary_color_lookup",
                "secondary_color_lookup",
            ],
        )
        .unwrap()
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PlayerLoadout::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::PlayerLoadoutRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PlayerLoadout::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
