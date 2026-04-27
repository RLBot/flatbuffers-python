use crate::{FromGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct PredictionSlice {
    /// The moment in game time that this prediction corresponds to.
    /// This corresponds to 'seconds_elapsed' in the MatchInfo.
    pub game_seconds: Py<PyFloat>,
    /// The predicted location and motion of the object.
    pub physics: Py<super::Physics>,
}

impl crate::PyDefault for PredictionSlice {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                game_seconds: crate::pyfloat_default(py),
                physics: super::Physics::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PredictionSlice> for PredictionSlice {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PredictionSlice) -> Self {
        Self {
            game_seconds: crate::float_to_py(py, flat_t.game_seconds),
            physics: crate::into_py_from(py, &flat_t.physics),
        }
    }
}

impl FromGil<&PredictionSlice> for flat::PredictionSlice {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PredictionSlice) -> Self {
        Self {
            game_seconds: crate::float_from_py(py, &py_type.game_seconds),
            physics: crate::from_py_into(py, &py_type.physics),
        }
    }
}

fn read_as_root<'a>(slice: &'a [u8]) -> ::planus::Result<flat::PredictionSliceRef<'a>> {
    planus::TableRead::from_buffer(
        planus::SliceWithStartOffset {
            buffer: slice,
            offset_from_start: 0,
        },
        0,
    )
    .map_err(|error_kind| error_kind.with_error_location("[PredictionSliceRef]", "read_as_root", 0))
}

#[pymethods]
impl PredictionSlice {
    #[new]
    #[pyo3(signature = (game_seconds=0.0, physics=None))]
    pub fn new(py: Python, game_seconds: f64, physics: Option<Py<super::Physics>>) -> Self {
        Self {
            game_seconds: PyFloat::new(py, game_seconds).unbind(),
            physics: physics.unwrap_or_else(|| super::Physics::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "PredictionSlice(game_seconds={}, physics={})",
            self.game_seconds,
            self.physics.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("game_seconds", "physics")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PredictionSlice::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PredictionSlice::from(flat_t_ref);

        Ok(crate::into_py_from(py, &flat_t))
    }
}
