use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct LoadoutPaint {
    #[pyo3(set)]
    pub car_paint_id: u32,
    #[pyo3(set)]
    pub decal_paint_id: u32,
    #[pyo3(set)]
    pub wheels_paint_id: u32,
    #[pyo3(set)]
    pub boost_paint_id: u32,
    #[pyo3(set)]
    pub antenna_paint_id: u32,
    #[pyo3(set)]
    pub hat_paint_id: u32,
    #[pyo3(set)]
    pub trails_paint_id: u32,
    #[pyo3(set)]
    pub goal_explosion_paint_id: u32,
}

impl crate::PyDefault for LoadoutPaint {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                car_paint_id: Default::default(),
                decal_paint_id: Default::default(),
                wheels_paint_id: Default::default(),
                boost_paint_id: Default::default(),
                antenna_paint_id: Default::default(),
                hat_paint_id: Default::default(),
                trails_paint_id: Default::default(),
                goal_explosion_paint_id: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::LoadoutPaint> for LoadoutPaint {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::LoadoutPaint) -> Self {
        LoadoutPaint {
            car_paint_id: flat_t.car_paint_id,
            decal_paint_id: flat_t.decal_paint_id,
            wheels_paint_id: flat_t.wheels_paint_id,
            boost_paint_id: flat_t.boost_paint_id,
            antenna_paint_id: flat_t.antenna_paint_id,
            hat_paint_id: flat_t.hat_paint_id,
            trails_paint_id: flat_t.trails_paint_id,
            goal_explosion_paint_id: flat_t.goal_explosion_paint_id,
        }
    }
}

impl FromGil<&LoadoutPaint> for flat::LoadoutPaint {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &LoadoutPaint) -> Self {
        Self {
            car_paint_id: py_type.car_paint_id,
            decal_paint_id: py_type.decal_paint_id,
            wheels_paint_id: py_type.wheels_paint_id,
            boost_paint_id: py_type.boost_paint_id,
            antenna_paint_id: py_type.antenna_paint_id,
            hat_paint_id: py_type.hat_paint_id,
            trails_paint_id: py_type.trails_paint_id,
            goal_explosion_paint_id: py_type.goal_explosion_paint_id,
        }
    }
}

#[pymethods]
impl LoadoutPaint {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (car_paint_id=0, decal_paint_id=0, wheels_paint_id=0, boost_paint_id=0, antenna_paint_id=0, hat_paint_id=0, trails_paint_id=0, goal_explosion_paint_id=0))]
    pub fn new(
        car_paint_id: u32,
        decal_paint_id: u32,
        wheels_paint_id: u32,
        boost_paint_id: u32,
        antenna_paint_id: u32,
        hat_paint_id: u32,
        trails_paint_id: u32,
        goal_explosion_paint_id: u32,
    ) -> Self {
        Self {
            car_paint_id,
            decal_paint_id,
            wheels_paint_id,
            boost_paint_id,
            antenna_paint_id,
            hat_paint_id,
            trails_paint_id,
            goal_explosion_paint_id,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "LoadoutPaint(car_paint_id={}, decal_paint_id={}, wheels_paint_id={}, boost_paint_id={}, antenna_paint_id={}, hat_paint_id={}, trails_paint_id={}, goal_explosion_paint_id={})",
            self.car_paint_id,
            self.decal_paint_id,
            self.wheels_paint_id,
            self.boost_paint_id,
            self.antenna_paint_id,
            self.hat_paint_id,
            self.trails_paint_id,
            self.goal_explosion_paint_id,
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
    ) {
        (
            "car_paint_id",
            "decal_paint_id",
            "wheels_paint_id",
            "boost_paint_id",
            "antenna_paint_id",
            "hat_paint_id",
            "trails_paint_id",
            "goal_explosion_paint_id",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::LoadoutPaint::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::LoadoutPaintRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::LoadoutPaint::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
