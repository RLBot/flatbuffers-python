use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct MatchInfo {
    pub seconds_elapsed: Py<PyFloat>,
    pub game_time_remaining: Py<PyFloat>,
    pub is_overtime: bool,
    pub is_unlimited_time: bool,
    pub match_phase: super::MatchPhase,
    pub world_gravity_z: Py<PyFloat>,
    pub game_speed: Py<PyFloat>,
    pub last_spectated: u32,
    pub frame_num: u32,
}

impl crate::PyDefault for MatchInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                seconds_elapsed: crate::pyfloat_default(py),
                game_time_remaining: crate::pyfloat_default(py),
                is_overtime: Default::default(),
                is_unlimited_time: Default::default(),
                match_phase: Default::default(),
                world_gravity_z: crate::pyfloat_default(py),
                game_speed: crate::pyfloat_default(py),
                last_spectated: Default::default(),
                frame_num: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::MatchInfo> for MatchInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::MatchInfo) -> Self {
        MatchInfo {
            seconds_elapsed: crate::float_to_py(py, flat_t.seconds_elapsed),
            game_time_remaining: crate::float_to_py(py, flat_t.game_time_remaining),
            is_overtime: flat_t.is_overtime,
            is_unlimited_time: flat_t.is_unlimited_time,
            match_phase: flat_t.match_phase.into(),
            world_gravity_z: crate::float_to_py(py, flat_t.world_gravity_z),
            game_speed: crate::float_to_py(py, flat_t.game_speed),
            last_spectated: flat_t.last_spectated,
            frame_num: flat_t.frame_num,
        }
    }
}

impl FromGil<&MatchInfo> for flat::MatchInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &MatchInfo) -> Self {
        Self {
            seconds_elapsed: crate::float_from_py(py, &py_type.seconds_elapsed),
            game_time_remaining: crate::float_from_py(py, &py_type.game_time_remaining),
            is_overtime: py_type.is_overtime,
            is_unlimited_time: py_type.is_unlimited_time,
            match_phase: (&py_type.match_phase).into(),
            world_gravity_z: crate::float_from_py(py, &py_type.world_gravity_z),
            game_speed: crate::float_from_py(py, &py_type.game_speed),
            last_spectated: py_type.last_spectated,
            frame_num: py_type.frame_num,
        }
    }
}

#[pymethods]
impl MatchInfo {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (seconds_elapsed=0.0, game_time_remaining=0.0, is_overtime=false, is_unlimited_time=false, match_phase=Default::default(), world_gravity_z=0.0, game_speed=0.0, last_spectated=0, frame_num=0))]
    pub fn new(
        py: Python,
        seconds_elapsed: f64,
        game_time_remaining: f64,
        is_overtime: bool,
        is_unlimited_time: bool,
        match_phase: super::MatchPhase,
        world_gravity_z: f64,
        game_speed: f64,
        last_spectated: u32,
        frame_num: u32,
    ) -> Self {
        Self {
            seconds_elapsed: PyFloat::new(py, seconds_elapsed).unbind(),
            game_time_remaining: PyFloat::new(py, game_time_remaining).unbind(),
            is_overtime,
            is_unlimited_time,
            match_phase,
            world_gravity_z: PyFloat::new(py, world_gravity_z).unbind(),
            game_speed: PyFloat::new(py, game_speed).unbind(),
            last_spectated,
            frame_num,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "MatchInfo(seconds_elapsed={}, game_time_remaining={}, is_overtime={}, is_unlimited_time={}, match_phase={}, world_gravity_z={}, game_speed={}, last_spectated={}, frame_num={})",
            self.seconds_elapsed,
            self.game_time_remaining,
            crate::bool_to_str(self.is_overtime),
            crate::bool_to_str(self.is_unlimited_time),
            self.match_phase.__repr__(),
            self.world_gravity_z,
            self.game_speed,
            self.last_spectated,
            self.frame_num,
        )
    }

    #[classattr]
    fn __match_args__() -> (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ) {
        (
            "seconds_elapsed",
            "game_time_remaining",
            "is_overtime",
            "is_unlimited_time",
            "match_phase",
            "world_gravity_z",
            "game_speed",
            "last_spectated",
            "frame_num",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::MatchInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::MatchInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::MatchInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
