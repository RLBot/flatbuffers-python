use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct MatchConfiguration {
    #[pyo3(set)]
    pub launcher: super::Launcher,
    #[pyo3(set)]
    pub launcher_arg: Py<PyString>,
    #[pyo3(set)]
    pub auto_start_agents: bool,
    #[pyo3(set)]
    pub wait_for_agents: bool,
    #[pyo3(set)]
    pub game_map_upk: Py<PyString>,
    #[pyo3(set)]
    pub player_configurations: Py<PyList>,
    #[pyo3(set)]
    pub script_configurations: Py<PyList>,
    #[pyo3(set)]
    pub game_mode: super::GameMode,
    #[pyo3(set)]
    pub skip_replays: bool,
    #[pyo3(set)]
    pub instant_start: bool,
    #[pyo3(set)]
    pub mutators: Option<Py<super::MutatorSettings>>,
    #[pyo3(set)]
    pub existing_match_behavior: super::ExistingMatchBehavior,
    #[pyo3(set)]
    pub enable_rendering: super::DebugRendering,
    #[pyo3(set)]
    pub enable_state_setting: bool,
    #[pyo3(set)]
    pub auto_save_replay: bool,
    #[pyo3(set)]
    pub freeplay: bool,
}

impl crate::PyDefault for MatchConfiguration {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                launcher: Default::default(),
                launcher_arg: crate::pydefault_string(py),
                auto_start_agents: Default::default(),
                wait_for_agents: Default::default(),
                game_map_upk: crate::pydefault_string(py),
                player_configurations: PyList::empty(py).unbind(),
                script_configurations: PyList::empty(py).unbind(),
                game_mode: Default::default(),
                skip_replays: Default::default(),
                instant_start: Default::default(),
                mutators: None,
                existing_match_behavior: Default::default(),
                enable_rendering: Default::default(),
                enable_state_setting: Default::default(),
                auto_save_replay: Default::default(),
                freeplay: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::MatchConfiguration> for MatchConfiguration {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::MatchConfiguration) -> Self {
        MatchConfiguration {
            launcher: flat_t.launcher,
            launcher_arg: PyString::new(py, &flat_t.launcher_arg).unbind(),
            auto_start_agents: flat_t.auto_start_agents,
            wait_for_agents: flat_t.wait_for_agents,
            game_map_upk: PyString::new(py, &flat_t.game_map_upk).unbind(),
            player_configurations: PyList::new(
                py,
                flat_t
                    .player_configurations
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::PlayerConfiguration>(py, x)),
            )
            .unwrap()
            .unbind(),
            script_configurations: PyList::new(
                py,
                flat_t
                    .script_configurations
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::ScriptConfiguration>(py, x)),
            )
            .unwrap()
            .unbind(),
            game_mode: flat_t.game_mode,
            skip_replays: flat_t.skip_replays,
            instant_start: flat_t.instant_start,
            mutators: flat_t
                .mutators
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            existing_match_behavior: flat_t.existing_match_behavior,
            enable_rendering: flat_t.enable_rendering,
            enable_state_setting: flat_t.enable_state_setting,
            auto_save_replay: flat_t.auto_save_replay,
            freeplay: flat_t.freeplay,
        }
    }
}

impl FromGil<&MatchConfiguration> for flat::MatchConfiguration {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &MatchConfiguration) -> Self {
        Self {
            launcher: py_type.launcher,
            launcher_arg: py_type.launcher_arg.to_str(py).unwrap().to_string(),
            auto_start_agents: py_type.auto_start_agents,
            wait_for_agents: py_type.wait_for_agents,
            game_map_upk: py_type.game_map_upk.to_str(py).unwrap().to_string(),
            player_configurations: py_type
                .player_configurations
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            script_configurations: py_type
                .script_configurations
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            game_mode: py_type.game_mode,
            skip_replays: py_type.skip_replays,
            instant_start: py_type.instant_start,
            mutators: py_type
                .mutators
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            existing_match_behavior: py_type.existing_match_behavior,
            enable_rendering: py_type.enable_rendering,
            enable_state_setting: py_type.enable_state_setting,
            auto_save_replay: py_type.auto_save_replay,
            freeplay: py_type.freeplay,
        }
    }
}

#[pymethods]
impl MatchConfiguration {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (launcher=Default::default(), launcher_arg=None, auto_start_agents=false, wait_for_agents=false, game_map_upk=None, player_configurations=None, script_configurations=None, game_mode=Default::default(), skip_replays=false, instant_start=false, mutators=None, existing_match_behavior=Default::default(), enable_rendering=Default::default(), enable_state_setting=false, auto_save_replay=false, freeplay=false))]
    pub fn new(
        py: Python,
        launcher: super::Launcher,
        launcher_arg: Option<Py<PyString>>,
        auto_start_agents: bool,
        wait_for_agents: bool,
        game_map_upk: Option<Py<PyString>>,
        player_configurations: Option<Py<PyList>>,
        script_configurations: Option<Py<PyList>>,
        game_mode: super::GameMode,
        skip_replays: bool,
        instant_start: bool,
        mutators: Option<Py<super::MutatorSettings>>,
        existing_match_behavior: super::ExistingMatchBehavior,
        enable_rendering: super::DebugRendering,
        enable_state_setting: bool,
        auto_save_replay: bool,
        freeplay: bool,
    ) -> Self {
        Self {
            launcher,
            launcher_arg: launcher_arg.unwrap_or_else(|| crate::pydefault_string(py)),
            auto_start_agents,
            wait_for_agents,
            game_map_upk: game_map_upk.unwrap_or_else(|| crate::pydefault_string(py)),
            player_configurations: player_configurations
                .unwrap_or_else(|| PyList::empty(py).unbind()),
            script_configurations: script_configurations
                .unwrap_or_else(|| PyList::empty(py).unbind()),
            game_mode,
            skip_replays,
            instant_start,
            mutators,
            existing_match_behavior,
            enable_rendering,
            enable_state_setting,
            auto_save_replay,
            freeplay,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "MatchConfiguration(launcher={}, launcher_arg={:?}, auto_start_agents={}, wait_for_agents={}, game_map_upk={:?}, player_configurations=[{}], script_configurations=[{}], game_mode={}, skip_replays={}, instant_start={}, mutators={}, existing_match_behavior={}, enable_rendering={}, enable_state_setting={}, auto_save_replay={}, freeplay={})",
            self.launcher.__repr__(),
            self.launcher_arg.bind(py).to_cow().unwrap(),
            crate::bool_to_str(self.auto_start_agents),
            crate::bool_to_str(self.wait_for_agents),
            self.game_map_upk.bind(py).to_cow().unwrap(),
            self.player_configurations
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::PlayerConfiguration>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.script_configurations
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::ScriptConfiguration>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.game_mode.__repr__(),
            crate::bool_to_str(self.skip_replays),
            crate::bool_to_str(self.instant_start),
            self.mutators
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.existing_match_behavior.__repr__(),
            self.enable_rendering.__repr__(),
            crate::bool_to_str(self.enable_state_setting),
            crate::bool_to_str(self.auto_save_replay),
            crate::bool_to_str(self.freeplay),
        )
    }

    #[classattr]
    fn __match_args__(py: Python) -> Bound<pyo3::types::PyTuple> {
        pyo3::types::PyTuple::new(
            py,
            [
                "launcher",
                "launcher_arg",
                "auto_start_agents",
                "wait_for_agents",
                "game_map_upk",
                "player_configurations",
                "script_configurations",
                "game_mode",
                "skip_replays",
                "instant_start",
                "mutators",
                "existing_match_behavior",
                "enable_rendering",
                "enable_state_setting",
                "auto_save_replay",
                "freeplay",
            ],
        )
        .unwrap()
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::MatchConfiguration::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::MatchConfigurationRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::MatchConfiguration::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
