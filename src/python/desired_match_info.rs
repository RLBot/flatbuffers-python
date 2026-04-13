use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct DesiredMatchInfo {
    pub world_gravity_z: Option<Py<PyFloat>>,
    pub game_speed: Option<Py<PyFloat>>,
}

impl crate::PyDefault for DesiredMatchInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                world_gravity_z: None,
                game_speed: None,
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::DesiredMatchInfo> for DesiredMatchInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::DesiredMatchInfo) -> Self {
        DesiredMatchInfo {
            world_gravity_z: flat_t
                .world_gravity_z
                .map(|x| crate::float_to_py(py, x.val)),
            game_speed: flat_t.game_speed.map(|x| crate::float_to_py(py, x.val)),
        }
    }
}

impl FromGil<&DesiredMatchInfo> for flat::DesiredMatchInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &DesiredMatchInfo) -> Self {
        Self {
            world_gravity_z: py_type.world_gravity_z.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
            game_speed: py_type.game_speed.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
        }
    }
}

#[pymethods]
impl DesiredMatchInfo {
    #[new]
    #[pyo3(signature = (world_gravity_z=None, game_speed=None))]
    pub fn new(py: Python, world_gravity_z: Option<f64>, game_speed: Option<f64>) -> Self {
        Self {
            world_gravity_z: world_gravity_z.map(|x| PyFloat::new(py, x).unbind()),
            game_speed: game_speed.map(|x| PyFloat::new(py, x).unbind()),
        }
    }

    #[setter]
    pub fn world_gravity_z(&mut self, py: Python, value: Option<f64>) {
        self.world_gravity_z = value.map(|x| PyFloat::new(py, x).unbind());
    }

    #[setter]
    pub fn game_speed(&mut self, py: Python, value: Option<f64>) {
        self.game_speed = value.map(|x| PyFloat::new(py, x).unbind());
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "DesiredMatchInfo(world_gravity_z={}, game_speed={})",
            self.world_gravity_z
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
            self.game_speed
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("world_gravity_z", "game_speed")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::DesiredMatchInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::DesiredMatchInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::DesiredMatchInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
