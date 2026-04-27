use crate::{FromGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct Rotator {
    /// In radians with range (-pi/2,+pi/2) where 0 is flat, +pi/2 is nose straight up, -pi/2 is nose straight down.
    pub pitch: Py<PyFloat>,
    /// In radians with range [-pi,+pi) where 0 is towards positive x, rotating clockwise as increased (when seen from above).
    pub yaw: Py<PyFloat>,
    /// In radians with range (-pi,+pi) where 0 is upright, positive is tilted right, negative is tilted left.
    pub roll: Py<PyFloat>,
}

impl crate::PyDefault for Rotator {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                pitch: crate::pyfloat_default(py),
                yaw: crate::pyfloat_default(py),
                roll: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Rotator> for Rotator {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Rotator) -> Self {
        Self {
            pitch: crate::float_to_py(py, flat_t.pitch),
            yaw: crate::float_to_py(py, flat_t.yaw),
            roll: crate::float_to_py(py, flat_t.roll),
        }
    }
}

impl FromGil<&Rotator> for flat::Rotator {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Rotator) -> Self {
        Self {
            pitch: crate::float_from_py(py, &py_type.pitch),
            yaw: crate::float_from_py(py, &py_type.yaw),
            roll: crate::float_from_py(py, &py_type.roll),
        }
    }
}

fn read_as_root<'a>(slice: &'a [u8]) -> ::planus::Result<flat::RotatorRef<'a>> {
    planus::TableRead::from_buffer(
        planus::SliceWithStartOffset {
            buffer: slice,
            offset_from_start: 0,
        },
        0,
    )
    .map_err(|error_kind| error_kind.with_error_location("[RotatorRef]", "read_as_root", 0))
}

#[pymethods]
impl Rotator {
    #[new]
    #[pyo3(signature = (pitch=0.0, yaw=0.0, roll=0.0))]
    pub fn new(py: Python, pitch: f64, yaw: f64, roll: f64) -> Self {
        Self {
            pitch: PyFloat::new(py, pitch).unbind(),
            yaw: PyFloat::new(py, yaw).unbind(),
            roll: PyFloat::new(py, roll).unbind(),
        }
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
            "Rotator(pitch={}, yaw={}, roll={})",
            self.pitch, self.yaw, self.roll,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("pitch", "yaw", "roll")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Rotator::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Rotator::from(flat_t_ref);

        Ok(crate::into_py_from(py, &flat_t))
    }
}
