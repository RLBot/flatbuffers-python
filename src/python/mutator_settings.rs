use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct MutatorSettings {
    #[pyo3(set)]
    pub match_length: super::MatchLengthMutator,
    #[pyo3(set)]
    pub max_score: super::MaxScoreMutator,
    #[pyo3(set)]
    pub multi_ball: super::MultiBallMutator,
    #[pyo3(set)]
    pub overtime: super::OvertimeMutator,
    #[pyo3(set)]
    pub series_length: super::SeriesLengthMutator,
    #[pyo3(set)]
    pub game_speed: super::GameSpeedMutator,
    #[pyo3(set)]
    pub ball_max_speed: super::BallMaxSpeedMutator,
    #[pyo3(set)]
    pub ball_type: super::BallTypeMutator,
    #[pyo3(set)]
    pub ball_weight: super::BallWeightMutator,
    #[pyo3(set)]
    pub ball_size: super::BallSizeMutator,
    #[pyo3(set)]
    pub ball_bounciness: super::BallBouncinessMutator,
    #[pyo3(set)]
    pub boost_amount: super::BoostAmountMutator,
    #[pyo3(set)]
    pub rumble: super::RumbleMutator,
    #[pyo3(set)]
    pub boost_strength: super::BoostStrengthMutator,
    #[pyo3(set)]
    pub gravity: super::GravityMutator,
    #[pyo3(set)]
    pub demolish: super::DemolishMutator,
    #[pyo3(set)]
    pub respawn_time: super::RespawnTimeMutator,
    #[pyo3(set)]
    pub max_time: super::MaxTimeMutator,
    #[pyo3(set)]
    pub game_event: super::GameEventMutator,
    #[pyo3(set)]
    pub audio: super::AudioMutator,
    #[pyo3(set)]
    pub ball_gravity: super::BallGravityMutator,
    #[pyo3(set)]
    pub territory: super::TerritoryMutator,
    #[pyo3(set)]
    pub stale_ball: super::StaleBallMutator,
    #[pyo3(set)]
    pub jump: super::JumpMutator,
    #[pyo3(set)]
    pub dodge_timer: super::DodgeTimerMutator,
    #[pyo3(set)]
    pub possession_score: super::PossessionScoreMutator,
    #[pyo3(set)]
    pub demolish_score: super::DemolishScoreMutator,
    #[pyo3(set)]
    pub normal_goal_score: super::NormalGoalScoreMutator,
    #[pyo3(set)]
    pub aerial_goal_score: super::AerialGoalScoreMutator,
    #[pyo3(set)]
    pub assist_goal_score: super::AssistGoalScoreMutator,
    #[pyo3(set)]
    pub input_restriction: super::InputRestrictionMutator,
    #[pyo3(set)]
    pub scoring_rule: super::ScoringRuleMutator,
}

impl crate::PyDefault for MutatorSettings {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                match_length: Default::default(),
                max_score: Default::default(),
                multi_ball: Default::default(),
                overtime: Default::default(),
                series_length: Default::default(),
                game_speed: Default::default(),
                ball_max_speed: Default::default(),
                ball_type: Default::default(),
                ball_weight: Default::default(),
                ball_size: Default::default(),
                ball_bounciness: Default::default(),
                boost_amount: Default::default(),
                rumble: Default::default(),
                boost_strength: Default::default(),
                gravity: Default::default(),
                demolish: Default::default(),
                respawn_time: Default::default(),
                max_time: Default::default(),
                game_event: Default::default(),
                audio: Default::default(),
                ball_gravity: Default::default(),
                territory: Default::default(),
                stale_ball: Default::default(),
                jump: Default::default(),
                dodge_timer: Default::default(),
                possession_score: Default::default(),
                demolish_score: Default::default(),
                normal_goal_score: Default::default(),
                aerial_goal_score: Default::default(),
                assist_goal_score: Default::default(),
                input_restriction: Default::default(),
                scoring_rule: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::MutatorSettings> for MutatorSettings {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::MutatorSettings) -> Self {
        MutatorSettings {
            match_length: flat_t.match_length,
            max_score: flat_t.max_score,
            multi_ball: flat_t.multi_ball,
            overtime: flat_t.overtime,
            series_length: flat_t.series_length,
            game_speed: flat_t.game_speed,
            ball_max_speed: flat_t.ball_max_speed,
            ball_type: flat_t.ball_type,
            ball_weight: flat_t.ball_weight,
            ball_size: flat_t.ball_size,
            ball_bounciness: flat_t.ball_bounciness,
            boost_amount: flat_t.boost_amount,
            rumble: flat_t.rumble,
            boost_strength: flat_t.boost_strength,
            gravity: flat_t.gravity,
            demolish: flat_t.demolish,
            respawn_time: flat_t.respawn_time,
            max_time: flat_t.max_time,
            game_event: flat_t.game_event,
            audio: flat_t.audio,
            ball_gravity: flat_t.ball_gravity,
            territory: flat_t.territory,
            stale_ball: flat_t.stale_ball,
            jump: flat_t.jump,
            dodge_timer: flat_t.dodge_timer,
            possession_score: flat_t.possession_score,
            demolish_score: flat_t.demolish_score,
            normal_goal_score: flat_t.normal_goal_score,
            aerial_goal_score: flat_t.aerial_goal_score,
            assist_goal_score: flat_t.assist_goal_score,
            input_restriction: flat_t.input_restriction,
            scoring_rule: flat_t.scoring_rule,
        }
    }
}

impl FromGil<&MutatorSettings> for flat::MutatorSettings {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &MutatorSettings) -> Self {
        Self {
            match_length: py_type.match_length,
            max_score: py_type.max_score,
            multi_ball: py_type.multi_ball,
            overtime: py_type.overtime,
            series_length: py_type.series_length,
            game_speed: py_type.game_speed,
            ball_max_speed: py_type.ball_max_speed,
            ball_type: py_type.ball_type,
            ball_weight: py_type.ball_weight,
            ball_size: py_type.ball_size,
            ball_bounciness: py_type.ball_bounciness,
            boost_amount: py_type.boost_amount,
            rumble: py_type.rumble,
            boost_strength: py_type.boost_strength,
            gravity: py_type.gravity,
            demolish: py_type.demolish,
            respawn_time: py_type.respawn_time,
            max_time: py_type.max_time,
            game_event: py_type.game_event,
            audio: py_type.audio,
            ball_gravity: py_type.ball_gravity,
            territory: py_type.territory,
            stale_ball: py_type.stale_ball,
            jump: py_type.jump,
            dodge_timer: py_type.dodge_timer,
            possession_score: py_type.possession_score,
            demolish_score: py_type.demolish_score,
            normal_goal_score: py_type.normal_goal_score,
            aerial_goal_score: py_type.aerial_goal_score,
            assist_goal_score: py_type.assist_goal_score,
            input_restriction: py_type.input_restriction,
            scoring_rule: py_type.scoring_rule,
        }
    }
}

#[pymethods]
impl MutatorSettings {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (match_length=Default::default(), max_score=Default::default(), multi_ball=Default::default(), overtime=Default::default(), series_length=Default::default(), game_speed=Default::default(), ball_max_speed=Default::default(), ball_type=Default::default(), ball_weight=Default::default(), ball_size=Default::default(), ball_bounciness=Default::default(), boost_amount=Default::default(), rumble=Default::default(), boost_strength=Default::default(), gravity=Default::default(), demolish=Default::default(), respawn_time=Default::default(), max_time=Default::default(), game_event=Default::default(), audio=Default::default(), ball_gravity=Default::default(), territory=Default::default(), stale_ball=Default::default(), jump=Default::default(), dodge_timer=Default::default(), possession_score=Default::default(), demolish_score=Default::default(), normal_goal_score=Default::default(), aerial_goal_score=Default::default(), assist_goal_score=Default::default(), input_restriction=Default::default(), scoring_rule=Default::default()))]
    pub fn new(
        match_length: super::MatchLengthMutator,
        max_score: super::MaxScoreMutator,
        multi_ball: super::MultiBallMutator,
        overtime: super::OvertimeMutator,
        series_length: super::SeriesLengthMutator,
        game_speed: super::GameSpeedMutator,
        ball_max_speed: super::BallMaxSpeedMutator,
        ball_type: super::BallTypeMutator,
        ball_weight: super::BallWeightMutator,
        ball_size: super::BallSizeMutator,
        ball_bounciness: super::BallBouncinessMutator,
        boost_amount: super::BoostAmountMutator,
        rumble: super::RumbleMutator,
        boost_strength: super::BoostStrengthMutator,
        gravity: super::GravityMutator,
        demolish: super::DemolishMutator,
        respawn_time: super::RespawnTimeMutator,
        max_time: super::MaxTimeMutator,
        game_event: super::GameEventMutator,
        audio: super::AudioMutator,
        ball_gravity: super::BallGravityMutator,
        territory: super::TerritoryMutator,
        stale_ball: super::StaleBallMutator,
        jump: super::JumpMutator,
        dodge_timer: super::DodgeTimerMutator,
        possession_score: super::PossessionScoreMutator,
        demolish_score: super::DemolishScoreMutator,
        normal_goal_score: super::NormalGoalScoreMutator,
        aerial_goal_score: super::AerialGoalScoreMutator,
        assist_goal_score: super::AssistGoalScoreMutator,
        input_restriction: super::InputRestrictionMutator,
        scoring_rule: super::ScoringRuleMutator,
    ) -> Self {
        Self {
            match_length,
            max_score,
            multi_ball,
            overtime,
            series_length,
            game_speed,
            ball_max_speed,
            ball_type,
            ball_weight,
            ball_size,
            ball_bounciness,
            boost_amount,
            rumble,
            boost_strength,
            gravity,
            demolish,
            respawn_time,
            max_time,
            game_event,
            audio,
            ball_gravity,
            territory,
            stale_ball,
            jump,
            dodge_timer,
            possession_score,
            demolish_score,
            normal_goal_score,
            aerial_goal_score,
            assist_goal_score,
            input_restriction,
            scoring_rule,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "MutatorSettings(match_length={}, max_score={}, multi_ball={}, overtime={}, series_length={}, game_speed={}, ball_max_speed={}, ball_type={}, ball_weight={}, ball_size={}, ball_bounciness={}, boost_amount={}, rumble={}, boost_strength={}, gravity={}, demolish={}, respawn_time={}, max_time={}, game_event={}, audio={}, ball_gravity={}, territory={}, stale_ball={}, jump={}, dodge_timer={}, possession_score={}, demolish_score={}, normal_goal_score={}, aerial_goal_score={}, assist_goal_score={}, input_restriction={}, scoring_rule={})",
            self.match_length.__repr__(),
            self.max_score.__repr__(),
            self.multi_ball.__repr__(),
            self.overtime.__repr__(),
            self.series_length.__repr__(),
            self.game_speed.__repr__(),
            self.ball_max_speed.__repr__(),
            self.ball_type.__repr__(),
            self.ball_weight.__repr__(),
            self.ball_size.__repr__(),
            self.ball_bounciness.__repr__(),
            self.boost_amount.__repr__(),
            self.rumble.__repr__(),
            self.boost_strength.__repr__(),
            self.gravity.__repr__(),
            self.demolish.__repr__(),
            self.respawn_time.__repr__(),
            self.max_time.__repr__(),
            self.game_event.__repr__(),
            self.audio.__repr__(),
            self.ball_gravity.__repr__(),
            self.territory.__repr__(),
            self.stale_ball.__repr__(),
            self.jump.__repr__(),
            self.dodge_timer.__repr__(),
            self.possession_score.__repr__(),
            self.demolish_score.__repr__(),
            self.normal_goal_score.__repr__(),
            self.aerial_goal_score.__repr__(),
            self.assist_goal_score.__repr__(),
            self.input_restriction.__repr__(),
            self.scoring_rule.__repr__(),
        )
    }

    #[classattr]
    fn __match_args__(py: Python) -> Bound<pyo3::types::PyTuple> {
        pyo3::types::PyTuple::new(
            py,
            [
                "match_length",
                "max_score",
                "multi_ball",
                "overtime",
                "series_length",
                "game_speed",
                "ball_max_speed",
                "ball_type",
                "ball_weight",
                "ball_size",
                "ball_bounciness",
                "boost_amount",
                "rumble",
                "boost_strength",
                "gravity",
                "demolish",
                "respawn_time",
                "max_time",
                "game_event",
                "audio",
                "ball_gravity",
                "territory",
                "stale_ball",
                "jump",
                "dodge_timer",
                "possession_score",
                "demolish_score",
                "normal_goal_score",
                "aerial_goal_score",
                "assist_goal_score",
                "input_restriction",
                "scoring_rule",
            ],
        )
        .unwrap()
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::MutatorSettings::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::MutatorSettingsRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::MutatorSettings::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
