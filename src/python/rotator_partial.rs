use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct RotatorPartial {
    pub pitch: Option<Py<PyFloat>>,
    pub yaw: Option<Py<PyFloat>>,
    pub roll: Option<Py<PyFloat>>,
}

impl crate::PyDefault for RotatorPartial {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                pitch: None,
                yaw: None,
                roll: None,
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::RotatorPartial> for RotatorPartial {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::RotatorPartial) -> Self {
        RotatorPartial {
            pitch: flat_t.pitch.map(|x| crate::float_to_py(py, x.val)),
            yaw: flat_t.yaw.map(|x| crate::float_to_py(py, x.val)),
            roll: flat_t.roll.map(|x| crate::float_to_py(py, x.val)),
        }
    }
}

impl FromGil<&RotatorPartial> for flat::RotatorPartial {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &RotatorPartial) -> Self {
        Self {
            pitch: py_type.pitch.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
            yaw: py_type.yaw.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
            roll: py_type.roll.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
        }
    }
}

#[pymethods]
impl RotatorPartial {
    #[new]
    #[pyo3(signature = (pitch=None, yaw=None, roll=None))]
    pub fn new(py: Python, pitch: Option<f64>, yaw: Option<f64>, roll: Option<f64>) -> Self {
        Self {
            pitch: pitch.map(|x| PyFloat::new(py, x).unbind()),
            yaw: yaw.map(|x| PyFloat::new(py, x).unbind()),
            roll: roll.map(|x| PyFloat::new(py, x).unbind()),
        }
    }

    #[setter]
    pub fn pitch(&mut self, py: Python, value: Option<f64>) {
        self.pitch = value.map(|x| PyFloat::new(py, x).unbind());
    }

    #[setter]
    pub fn yaw(&mut self, py: Python, value: Option<f64>) {
        self.yaw = value.map(|x| PyFloat::new(py, x).unbind());
    }

    #[setter]
    pub fn roll(&mut self, py: Python, value: Option<f64>) {
        self.roll = value.map(|x| PyFloat::new(py, x).unbind());
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "RotatorPartial(pitch={}, yaw={}, roll={})",
            self.pitch
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
            self.yaw
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
            self.roll
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("pitch", "yaw", "roll")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::RotatorPartial::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::RotatorPartialRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::RotatorPartial::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
