use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct DesiredCarState {
    #[pyo3(set)]
    pub physics: Option<Py<super::DesiredPhysics>>,
    pub boost_amount: Option<Py<PyFloat>>,
}

impl crate::PyDefault for DesiredCarState {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                physics: None,
                boost_amount: None,
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::DesiredCarState> for DesiredCarState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::DesiredCarState) -> Self {
        DesiredCarState {
            physics: flat_t
                .physics
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            boost_amount: flat_t.boost_amount.map(|x| crate::float_to_py(py, x.val)),
        }
    }
}

impl FromGil<&DesiredCarState> for flat::DesiredCarState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &DesiredCarState) -> Self {
        Self {
            physics: py_type
                .physics
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            boost_amount: py_type.boost_amount.as_ref().map(|x| flat::Float {
                val: crate::float_from_py(py, x),
            }),
        }
    }
}

#[pymethods]
impl DesiredCarState {
    #[new]
    #[pyo3(signature = (physics=None, boost_amount=None))]
    pub fn new(
        py: Python,
        physics: Option<Py<super::DesiredPhysics>>,
        boost_amount: Option<f64>,
    ) -> Self {
        Self {
            physics,
            boost_amount: boost_amount.map(|x| PyFloat::new(py, x).unbind()),
        }
    }

    #[setter]
    pub fn boost_amount(&mut self, py: Python, value: Option<f64>) {
        self.boost_amount = value.map(|x| PyFloat::new(py, x).unbind());
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "DesiredCarState(physics={}, boost_amount={})",
            self.physics
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.boost_amount
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.to_string() }),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("physics", "boost_amount")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::DesiredCarState::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::DesiredCarStateRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::DesiredCarState::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
