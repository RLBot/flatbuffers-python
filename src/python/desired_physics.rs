use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct DesiredPhysics {
    #[pyo3(set)]
    pub location: Option<Py<super::Vector3Partial>>,
    #[pyo3(set)]
    pub rotation: Option<Py<super::RotatorPartial>>,
    #[pyo3(set)]
    pub velocity: Option<Py<super::Vector3Partial>>,
    #[pyo3(set)]
    pub angular_velocity: Option<Py<super::Vector3Partial>>,
}

impl crate::PyDefault for DesiredPhysics {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                location: None,
                rotation: None,
                velocity: None,
                angular_velocity: None,
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::DesiredPhysics> for DesiredPhysics {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::DesiredPhysics) -> Self {
        DesiredPhysics {
            location: flat_t
                .location
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            rotation: flat_t
                .rotation
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            velocity: flat_t
                .velocity
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            angular_velocity: flat_t
                .angular_velocity
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
        }
    }
}

impl FromGil<&DesiredPhysics> for flat::DesiredPhysics {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &DesiredPhysics) -> Self {
        Self {
            location: py_type
                .location
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            rotation: py_type
                .rotation
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            velocity: py_type
                .velocity
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            angular_velocity: py_type
                .angular_velocity
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
        }
    }
}

#[pymethods]
impl DesiredPhysics {
    #[new]
    #[pyo3(signature = (location=None, rotation=None, velocity=None, angular_velocity=None))]
    pub fn new(
        location: Option<Py<super::Vector3Partial>>,
        rotation: Option<Py<super::RotatorPartial>>,
        velocity: Option<Py<super::Vector3Partial>>,
        angular_velocity: Option<Py<super::Vector3Partial>>,
    ) -> Self {
        Self {
            location,
            rotation,
            velocity,
            angular_velocity,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "DesiredPhysics(location={}, rotation={}, velocity={}, angular_velocity={})",
            self.location
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.rotation
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.velocity
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.angular_velocity
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        ("location", "rotation", "velocity", "angular_velocity")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::DesiredPhysics::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::DesiredPhysicsRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::DesiredPhysics::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
