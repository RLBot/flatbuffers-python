use crate::{FromGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct ScoreInfo {
    /// The accumulated score, roughly indicating how well a player performs.
    pub score: u32,
    /// Number of goals scored.
    pub goals: u32,
    /// Number of own-goals scored.
    pub own_goals: u32,
    /// Number of goals assisted.
    pub assists: u32,
    /// Number of shots saved.
    pub saves: u32,
    /// Number of shots on opponent goal.
    pub shots: u32,
    /// Number of demolitions made.
    pub demolitions: u32,
}

impl crate::PyDefault for ScoreInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                score: Default::default(),
                goals: Default::default(),
                own_goals: Default::default(),
                assists: Default::default(),
                saves: Default::default(),
                shots: Default::default(),
                demolitions: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::ScoreInfo> for ScoreInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::ScoreInfo) -> Self {
        Self {
            score: flat_t.score,
            goals: flat_t.goals,
            own_goals: flat_t.own_goals,
            assists: flat_t.assists,
            saves: flat_t.saves,
            shots: flat_t.shots,
            demolitions: flat_t.demolitions,
        }
    }
}

impl FromGil<&ScoreInfo> for flat::ScoreInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &ScoreInfo) -> Self {
        Self {
            score: py_type.score,
            goals: py_type.goals,
            own_goals: py_type.own_goals,
            assists: py_type.assists,
            saves: py_type.saves,
            shots: py_type.shots,
            demolitions: py_type.demolitions,
        }
    }
}

fn read_as_root<'a>(slice: &'a [u8]) -> ::planus::Result<flat::ScoreInfoRef<'a>> {
    planus::TableRead::from_buffer(
        planus::SliceWithStartOffset {
            buffer: slice,
            offset_from_start: 0,
        },
        0,
    )
    .map_err(|error_kind| error_kind.with_error_location("[ScoreInfoRef]", "read_as_root", 0))
}

#[pymethods]
impl ScoreInfo {
    #[new]
    #[pyo3(signature = (score=0, goals=0, own_goals=0, assists=0, saves=0, shots=0, demolitions=0))]
    pub fn new(
        score: u32,
        goals: u32,
        own_goals: u32,
        assists: u32,
        saves: u32,
        shots: u32,
        demolitions: u32,
    ) -> Self {
        Self {
            score,
            goals,
            own_goals,
            assists,
            saves,
            shots,
            demolitions,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "ScoreInfo(score={}, goals={}, own_goals={}, assists={}, saves={}, shots={}, demolitions={})",
            self.score,
            self.goals,
            self.own_goals,
            self.assists,
            self.saves,
            self.shots,
            self.demolitions,
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
    ) {
        (
            "score",
            "goals",
            "own_goals",
            "assists",
            "saves",
            "shots",
            "demolitions",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::ScoreInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::ScoreInfo::from(flat_t_ref);

        Ok(crate::into_py_from(py, &flat_t))
    }
}
