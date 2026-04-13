use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct GoalInfo {
    pub team_num: i32,
    pub location: Py<super::Vector3>,
    pub direction: Py<super::Vector3>,
    pub width: Py<PyFloat>,
    pub height: Py<PyFloat>,
}

impl crate::PyDefault for GoalInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                team_num: Default::default(),
                location: super::Vector3::py_default(py),
                direction: super::Vector3::py_default(py),
                width: crate::pyfloat_default(py),
                height: crate::pyfloat_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::GoalInfo> for GoalInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::GoalInfo) -> Self {
        GoalInfo {
            team_num: flat_t.team_num,
            location: crate::into_py_from(py, &flat_t.location),
            direction: crate::into_py_from(py, &flat_t.direction),
            width: crate::float_to_py(py, flat_t.width),
            height: crate::float_to_py(py, flat_t.height),
        }
    }
}

impl FromGil<&GoalInfo> for flat::GoalInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &GoalInfo) -> Self {
        Self {
            team_num: py_type.team_num,
            location: crate::from_py_into(py, &py_type.location),
            direction: crate::from_py_into(py, &py_type.direction),
            width: crate::float_from_py(py, &py_type.width),
            height: crate::float_from_py(py, &py_type.height),
        }
    }
}

#[pymethods]
impl GoalInfo {
    #[new]
    #[pyo3(signature = (team_num=0, location=None, direction=None, width=0.0, height=0.0))]
    pub fn new(
        py: Python,
        team_num: i32,
        location: Option<Py<super::Vector3>>,
        direction: Option<Py<super::Vector3>>,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            team_num,
            location: location.unwrap_or_else(|| super::Vector3::py_default(py)),
            direction: direction.unwrap_or_else(|| super::Vector3::py_default(py)),
            width: PyFloat::new(py, width).unbind(),
            height: PyFloat::new(py, height).unbind(),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "GoalInfo(team_num={}, location={}, direction={}, width={}, height={})",
            self.team_num,
            self.location.borrow(py).__repr__(py),
            self.direction.borrow(py).__repr__(py),
            self.width,
            self.height,
        )
    }

    #[classattr]
    fn __match_args__() -> (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ) {
        ("team_num", "location", "direction", "width", "height")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::GoalInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::GoalInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::GoalInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
