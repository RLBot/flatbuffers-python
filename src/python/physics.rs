use crate::{FromGil, PyDefault, flat};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct Physics {
    pub location: Py<super::Vector3>,
    pub rotation: Py<super::Rotator>,
    pub velocity: Py<super::Vector3>,
    pub angular_velocity: Py<super::Vector3>,
}

impl crate::PyDefault for Physics {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                location: super::Vector3::py_default(py),
                rotation: super::Rotator::py_default(py),
                velocity: super::Vector3::py_default(py),
                angular_velocity: super::Vector3::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Physics> for Physics {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Physics) -> Self {
        Self {
            location: crate::into_py_from(py, &flat_t.location),
            rotation: crate::into_py_from(py, &flat_t.rotation),
            velocity: crate::into_py_from(py, &flat_t.velocity),
            angular_velocity: crate::into_py_from(py, &flat_t.angular_velocity),
        }
    }
}

impl FromGil<&Physics> for flat::Physics {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Physics) -> Self {
        Self {
            location: crate::from_py_into(py, &py_type.location),
            rotation: crate::from_py_into(py, &py_type.rotation),
            velocity: crate::from_py_into(py, &py_type.velocity),
            angular_velocity: crate::from_py_into(py, &py_type.angular_velocity),
        }
    }
}

#[pymethods]
impl Physics {
    #[new]
    #[pyo3(signature = (location=None, rotation=None, velocity=None, angular_velocity=None))]
    pub fn new(
        py: Python,
        location: Option<Py<super::Vector3>>,
        rotation: Option<Py<super::Rotator>>,
        velocity: Option<Py<super::Vector3>>,
        angular_velocity: Option<Py<super::Vector3>>,
    ) -> Self {
        Self {
            location: location.unwrap_or_else(|| super::Vector3::py_default(py)),
            rotation: rotation.unwrap_or_else(|| super::Rotator::py_default(py)),
            velocity: velocity.unwrap_or_else(|| super::Vector3::py_default(py)),
            angular_velocity: angular_velocity.unwrap_or_else(|| super::Vector3::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Physics(location={}, rotation={}, velocity={}, angular_velocity={})",
            self.location.borrow(py).__repr__(py),
            self.rotation.borrow(py).__repr__(py),
            self.velocity.borrow(py).__repr__(py),
            self.angular_velocity.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        ("location", "rotation", "velocity", "angular_velocity")
    }
}
