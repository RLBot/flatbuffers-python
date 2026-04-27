use crate::{FromGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct BoostPadState {
    /// True if the boost can be picked up right now.
    pub is_active: bool,
    /// The number of seconds since the boost has been picked up, or 0 if the boost is active.
    /// A big boost pad becomes active again after 10 seconds.
    /// A small boost pad becomes active again after 4 seconds.
    pub timer: Py<PyFloat>,
}

impl crate::PyDefault for BoostPadState {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                is_active: Default::default(),
                timer: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::BoostPadState> for BoostPadState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::BoostPadState) -> Self {
        Self {
            is_active: flat_t.is_active,
            timer: crate::float_to_py(py, flat_t.timer),
        }
    }
}

impl FromGil<&BoostPadState> for flat::BoostPadState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &BoostPadState) -> Self {
        Self {
            is_active: py_type.is_active,
            timer: crate::float_from_py(py, &py_type.timer),
        }
    }
}

fn read_as_root<'a>(slice: &'a [u8]) -> ::planus::Result<flat::BoostPadStateRef<'a>> {
    planus::TableRead::from_buffer(
        planus::SliceWithStartOffset {
            buffer: slice,
            offset_from_start: 0,
        },
        0,
    )
    .map_err(|error_kind| error_kind.with_error_location("[BoostPadStateRef]", "read_as_root", 0))
}

#[pymethods]
impl BoostPadState {
    #[new]
    #[pyo3(signature = (is_active=false, timer=0.0))]
    pub fn new(py: Python, is_active: bool, timer: f64) -> Self {
        Self {
            is_active,
            timer: PyFloat::new(py, timer).unbind(),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "BoostPadState(is_active={}, timer={})",
            crate::bool_to_str(self.is_active),
            self.timer,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("is_active", "timer")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::BoostPadState::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::BoostPadState::from(flat_t_ref);

        Ok(crate::into_py_from(py, &flat_t))
    }
}
