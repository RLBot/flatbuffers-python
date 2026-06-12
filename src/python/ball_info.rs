use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct BallInfo {
    pub physics: Py<super::Physics>,
    pub shape: Py<PyAny>,
    pub charge_level: i32,
    pub target_speed: Py<PyFloat>,
}

impl crate::PyDefault for BallInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                physics: super::Physics::py_default(py),
                shape: super::CollisionShape::py_default(py),
                charge_level: Default::default(),
                target_speed: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::BallInfo> for BallInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::BallInfo) -> Self {
        BallInfo {
            physics: crate::into_py_from(py, &flat_t.physics),
            shape: IntoGil::<super::CollisionShape>::into_gil(&flat_t.shape, py).into_any(),
            charge_level: flat_t.charge_level,
            target_speed: crate::float_to_py(py, flat_t.target_speed),
        }
    }
}

impl FromGil<&BallInfo> for flat::BallInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &BallInfo) -> Self {
        Self {
            physics: crate::from_py_into(py, &py_type.physics),
            shape: super::CollisionShape::extract(py_type.shape.bind_borrowed(py))
                .as_ref()
                .unwrap()
                .into_gil(py),
            charge_level: py_type.charge_level,
            target_speed: crate::float_from_py(py, &py_type.target_speed),
        }
    }
}

#[pymethods]
impl BallInfo {
    #[new]
    #[pyo3(signature = (physics=None, shape=None, charge_level=0, target_speed=0.0))]
    pub fn new(
        py: Python,
        physics: Option<Py<super::Physics>>,
        shape: Option<Py<PyAny>>,
        charge_level: i32,
        target_speed: f64,
    ) -> Self {
        Self {
            physics: physics.unwrap_or_else(|| super::Physics::py_default(py)),
            shape: shape.unwrap_or_else(|| super::CollisionShape::py_default(py)),
            charge_level,
            target_speed: PyFloat::new(py, target_speed).unbind(),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "BallInfo(physics={}, shape={}, charge_level={}, target_speed={})",
            self.physics.borrow(py).__repr__(py),
            super::CollisionShape::extract(self.shape.bind_borrowed(py))
                .unwrap()
                .__repr__(py),
            self.charge_level,
            self.target_speed,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        ("physics", "shape", "charge_level", "target_speed")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::BallInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::BallInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::BallInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
