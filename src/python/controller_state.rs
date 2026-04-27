use crate::{FromGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct ControllerState {
    /// -1 for full reverse, 1 for full forward.
    pub throttle: Py<PyFloat>,
    /// -1 for full left, 1 for full right.
    pub steer: Py<PyFloat>,
    /// -1 for nose down, 1 for nose up.
    pub pitch: Py<PyFloat>,
    /// -1 for full left, 1 for full right.
    pub yaw: Py<PyFloat>,
    /// -1 for roll left, 1 for roll right.
    pub roll: Py<PyFloat>,
    /// True if you want to press the jump button.
    #[pyo3(set)]
    pub jump: bool,
    /// True if you want to press the boost button.
    #[pyo3(set)]
    pub boost: bool,
    /// True if you want to press the handbrake button.
    #[pyo3(set)]
    pub handbrake: bool,
    /// True if you want to press the 'use item' button. Used in Rumble and other game modes.
    #[pyo3(set)]
    pub use_item: bool,
}

impl crate::PyDefault for ControllerState {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                throttle: crate::pyfloat_default(py),
                steer: crate::pyfloat_default(py),
                pitch: crate::pyfloat_default(py),
                yaw: crate::pyfloat_default(py),
                roll: crate::pyfloat_default(py),
                jump: Default::default(),
                boost: Default::default(),
                handbrake: Default::default(),
                use_item: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::ControllerState> for ControllerState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::ControllerState) -> Self {
        Self {
            throttle: crate::float_to_py(py, flat_t.throttle),
            steer: crate::float_to_py(py, flat_t.steer),
            pitch: crate::float_to_py(py, flat_t.pitch),
            yaw: crate::float_to_py(py, flat_t.yaw),
            roll: crate::float_to_py(py, flat_t.roll),
            jump: flat_t.jump,
            boost: flat_t.boost,
            handbrake: flat_t.handbrake,
            use_item: flat_t.use_item,
        }
    }
}

impl FromGil<&ControllerState> for flat::ControllerState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &ControllerState) -> Self {
        Self {
            throttle: crate::float_from_py(py, &py_type.throttle),
            steer: crate::float_from_py(py, &py_type.steer),
            pitch: crate::float_from_py(py, &py_type.pitch),
            yaw: crate::float_from_py(py, &py_type.yaw),
            roll: crate::float_from_py(py, &py_type.roll),
            jump: py_type.jump,
            boost: py_type.boost,
            handbrake: py_type.handbrake,
            use_item: py_type.use_item,
        }
    }
}

fn read_as_root<'a>(slice: &'a [u8]) -> ::planus::Result<flat::ControllerStateRef<'a>> {
    planus::TableRead::from_buffer(
        planus::SliceWithStartOffset {
            buffer: slice,
            offset_from_start: 0,
        },
        0,
    )
    .map_err(|error_kind| error_kind.with_error_location("[ControllerStateRef]", "read_as_root", 0))
}

#[pymethods]
impl ControllerState {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (throttle=0.0, steer=0.0, pitch=0.0, yaw=0.0, roll=0.0, jump=false, boost=false, handbrake=false, use_item=false))]
    pub fn new(
        py: Python,
        throttle: f64,
        steer: f64,
        pitch: f64,
        yaw: f64,
        roll: f64,
        jump: bool,
        boost: bool,
        handbrake: bool,
        use_item: bool,
    ) -> Self {
        Self {
            throttle: PyFloat::new(py, throttle).unbind(),
            steer: PyFloat::new(py, steer).unbind(),
            pitch: PyFloat::new(py, pitch).unbind(),
            yaw: PyFloat::new(py, yaw).unbind(),
            roll: PyFloat::new(py, roll).unbind(),
            jump,
            boost,
            handbrake,
            use_item,
        }
    }

    #[setter]
    pub fn throttle(&mut self, py: Python, value: f64) {
        self.throttle = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn steer(&mut self, py: Python, value: f64) {
        self.steer = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn pitch(&mut self, py: Python, value: f64) {
        self.pitch = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn yaw(&mut self, py: Python, value: f64) {
        self.yaw = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn roll(&mut self, py: Python, value: f64) {
        self.roll = PyFloat::new(py, value).unbind();
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "ControllerState(throttle={}, steer={}, pitch={}, yaw={}, roll={}, jump={}, boost={}, handbrake={}, use_item={})",
            self.throttle,
            self.steer,
            self.pitch,
            self.yaw,
            self.roll,
            crate::bool_to_str(self.jump),
            crate::bool_to_str(self.boost),
            crate::bool_to_str(self.handbrake),
            crate::bool_to_str(self.use_item),
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
            "throttle",
            "steer",
            "pitch",
            "yaw",
            "roll",
            "jump",
            "boost",
            "handbrake",
            "use_item",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::ControllerState::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::ControllerState::from(flat_t_ref);

        Ok(crate::into_py_from(py, &flat_t))
    }
}
