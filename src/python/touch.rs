use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct Touch {
    pub game_seconds: Py<PyFloat>,
    pub location: Py<super::Vector3>,
    pub normal: Py<super::Vector3>,
    pub ball_index: u32,
}

impl crate::PyDefault for Touch {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                game_seconds: crate::pyfloat_default(py),
                location: super::Vector3::py_default(py),
                normal: super::Vector3::py_default(py),
                ball_index: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Touch> for Touch {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Touch) -> Self {
        Touch {
            game_seconds: crate::float_to_py(py, flat_t.game_seconds),
            location: crate::into_py_from(py, &flat_t.location),
            normal: crate::into_py_from(py, &flat_t.normal),
            ball_index: flat_t.ball_index,
        }
    }
}

impl FromGil<&Touch> for flat::Touch {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Touch) -> Self {
        Self {
            game_seconds: crate::float_from_py(py, &py_type.game_seconds),
            location: crate::from_py_into(py, &py_type.location),
            normal: crate::from_py_into(py, &py_type.normal),
            ball_index: py_type.ball_index,
        }
    }
}

#[pymethods]
impl Touch {
    #[new]
    #[pyo3(signature = (game_seconds=0.0, location=None, normal=None, ball_index=0))]
    pub fn new(
        py: Python,
        game_seconds: f64,
        location: Option<Py<super::Vector3>>,
        normal: Option<Py<super::Vector3>>,
        ball_index: u32,
    ) -> Self {
        Self {
            game_seconds: PyFloat::new(py, game_seconds).unbind(),
            location: location.unwrap_or_else(|| super::Vector3::py_default(py)),
            normal: normal.unwrap_or_else(|| super::Vector3::py_default(py)),
            ball_index,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Touch(game_seconds={}, location={}, normal={}, ball_index={})",
            self.game_seconds,
            self.location.borrow(py).__repr__(py),
            self.normal.borrow(py).__repr__(py),
            self.ball_index,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        ("game_seconds", "location", "normal", "ball_index")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Touch::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::TouchRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Touch::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
