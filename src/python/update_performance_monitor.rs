use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct UpdatePerformanceMonitor {
    #[pyo3(set)]
    pub show: super::PerformanceMonitor,
}

impl crate::PyDefault for UpdatePerformanceMonitor {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                show: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::UpdatePerformanceMonitor> for UpdatePerformanceMonitor {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::UpdatePerformanceMonitor) -> Self {
        UpdatePerformanceMonitor { show: flat_t.show }
    }
}

impl FromGil<&UpdatePerformanceMonitor> for flat::UpdatePerformanceMonitor {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &UpdatePerformanceMonitor) -> Self {
        Self { show: py_type.show }
    }
}

#[pymethods]
impl UpdatePerformanceMonitor {
    #[new]
    #[pyo3(signature = (show=Default::default()))]
    pub fn new(show: super::PerformanceMonitor) -> Self {
        Self { show }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!("UpdatePerformanceMonitor(show={})", self.show.__repr__(),)
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("show",)
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::UpdatePerformanceMonitor::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref =
            flat::UpdatePerformanceMonitorRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t =
            flat::UpdatePerformanceMonitor::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
