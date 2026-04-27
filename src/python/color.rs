use crate::{FromGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct Color {
    #[pyo3(set)]
    pub r: u8,
    #[pyo3(set)]
    pub g: u8,
    #[pyo3(set)]
    pub b: u8,
    #[pyo3(set)]
    pub a: u8,
}

impl crate::PyDefault for Color {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                r: Default::default(),
                g: Default::default(),
                b: Default::default(),
                a: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Color> for Color {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Color) -> Self {
        Self {
            r: flat_t.r,
            g: flat_t.g,
            b: flat_t.b,
            a: flat_t.a,
        }
    }
}

impl FromGil<&Color> for flat::Color {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Color) -> Self {
        Self {
            r: py_type.r,
            g: py_type.g,
            b: py_type.b,
            a: py_type.a,
        }
    }
}

fn read_as_root<'a>(slice: &'a [u8]) -> ::planus::Result<flat::ColorRef<'a>> {
    planus::TableRead::from_buffer(
        planus::SliceWithStartOffset {
            buffer: slice,
            offset_from_start: 0,
        },
        0,
    )
    .map_err(|error_kind| error_kind.with_error_location("[ColorRef]", "read_as_root", 0))
}

#[pymethods]
impl Color {
    #[new]
    #[pyo3(signature = (r=0, g=0, b=0, a=255))]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Color(r={}, g={}, b={}, a={})",
            self.r, self.g, self.b, self.a,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        ("r", "g", "b", "a")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Color::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Color::from(flat_t_ref);

        Ok(crate::into_py_from(py, &flat_t))
    }
}
