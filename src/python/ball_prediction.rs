use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct BallPrediction {
    pub slices: Py<PyList>,
}

impl crate::PyDefault for BallPrediction {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                slices: PyList::empty(py).unbind(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::BallPrediction> for BallPrediction {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::BallPrediction) -> Self {
        BallPrediction {
            slices: PyList::new(
                py,
                flat_t
                    .slices
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::PredictionSlice>(py, x)),
            )
            .unwrap()
            .unbind(),
        }
    }
}

impl FromGil<&BallPrediction> for flat::BallPrediction {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &BallPrediction) -> Self {
        Self {
            slices: py_type
                .slices
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
        }
    }
}

#[pymethods]
impl BallPrediction {
    #[new]
    #[pyo3(signature = (slices=None))]
    pub fn new(py: Python, slices: Option<Py<PyList>>) -> Self {
        Self {
            slices: slices.unwrap_or_else(|| PyList::empty(py).unbind()),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "BallPrediction(slices=[{}])",
            self.slices
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::PredictionSlice>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("slices",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::BallPrediction::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::BallPredictionRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::BallPrediction::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
