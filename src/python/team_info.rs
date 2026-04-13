use crate::{FromGil, PyDefault, flat};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct TeamInfo {
    /// The index of the team. Blue is 0, orange is 1.
    pub team_index: u32,
    /// Number of goals scored.
    /// Note, this value may be different than the sum of the goals scored by the current players on the team as player may join/leave the game or switch teams.
    /// This value is what is shown on the heads-up display.
    pub score: u32,
}

impl crate::PyDefault for TeamInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                team_index: Default::default(),
                score: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::TeamInfo> for TeamInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::TeamInfo) -> Self {
        Self {
            team_index: flat_t.team_index,
            score: flat_t.score,
        }
    }
}

impl FromGil<&TeamInfo> for flat::TeamInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &TeamInfo) -> Self {
        Self {
            team_index: py_type.team_index,
            score: py_type.score,
        }
    }
}

#[pymethods]
impl TeamInfo {
    #[new]
    #[pyo3(signature = (team_index=0, score=0))]
    pub fn new(team_index: u32, score: u32) -> Self {
        Self { team_index, score }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "TeamInfo(team_index={}, score={})",
            self.team_index, self.score,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("team_index", "score")
    }
}
