use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct String2D {
    #[pyo3(set)]
    pub text: Py<PyString>,
    pub x: Py<PyFloat>,
    pub y: Py<PyFloat>,
    pub scale: Py<PyFloat>,
    #[pyo3(set)]
    pub foreground: Py<super::Color>,
    #[pyo3(set)]
    pub background: Py<super::Color>,
    #[pyo3(set)]
    pub h_align: super::TextHAlign,
    #[pyo3(set)]
    pub v_align: super::TextVAlign,
}

impl crate::PyDefault for String2D {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                text: crate::pydefault_string(py),
                x: crate::pyfloat_default(py),
                y: crate::pyfloat_default(py),
                scale: crate::pyfloat_default(py),
                foreground: super::Color::py_default(py),
                background: super::Color::py_default(py),
                h_align: Default::default(),
                v_align: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::String2D> for String2D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::String2D) -> Self {
        String2D {
            text: PyString::new(py, &flat_t.text).unbind(),
            x: crate::float_to_py(py, flat_t.x),
            y: crate::float_to_py(py, flat_t.y),
            scale: crate::float_to_py(py, flat_t.scale),
            foreground: crate::into_py_from(py, &flat_t.foreground),
            background: crate::into_py_from(py, &flat_t.background),
            h_align: flat_t.h_align.into(),
            v_align: flat_t.v_align.into(),
        }
    }
}

impl FromGil<&String2D> for flat::String2D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &String2D) -> Self {
        Self {
            text: py_type.text.to_str(py).unwrap().to_string(),
            x: crate::float_from_py(py, &py_type.x),
            y: crate::float_from_py(py, &py_type.y),
            scale: crate::float_from_py(py, &py_type.scale),
            foreground: crate::from_py_into(py, &py_type.foreground),
            background: crate::from_py_into(py, &py_type.background),
            h_align: (&py_type.h_align).into(),
            v_align: (&py_type.v_align).into(),
        }
    }
}

#[pymethods]
impl String2D {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (text=None, x=0.0, y=0.0, scale=0.0, foreground=None, background=None, h_align=Default::default(), v_align=Default::default()))]
    pub fn new(
        py: Python,
        text: Option<Py<PyString>>,
        x: f64,
        y: f64,
        scale: f64,
        foreground: Option<Py<super::Color>>,
        background: Option<Py<super::Color>>,
        h_align: super::TextHAlign,
        v_align: super::TextVAlign,
    ) -> Self {
        Self {
            text: text.unwrap_or_else(|| crate::pydefault_string(py)),
            x: PyFloat::new(py, x).unbind(),
            y: PyFloat::new(py, y).unbind(),
            scale: PyFloat::new(py, scale).unbind(),
            foreground: foreground.unwrap_or_else(|| super::Color::py_default(py)),
            background: background.unwrap_or_else(|| super::Color::py_default(py)),
            h_align,
            v_align,
        }
    }

    #[setter]
    pub fn x(&mut self, py: Python, value: f64) {
        self.x = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn y(&mut self, py: Python, value: f64) {
        self.y = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn scale(&mut self, py: Python, value: f64) {
        self.scale = PyFloat::new(py, value).unbind();
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "String2D(text={:?}, x={}, y={}, scale={}, foreground={}, background={}, h_align={}, v_align={})",
            self.text.bind(py).to_cow().unwrap(),
            self.x,
            self.y,
            self.scale,
            self.foreground.borrow(py).__repr__(py),
            self.background.borrow(py).__repr__(py),
            self.h_align.__repr__(),
            self.v_align.__repr__(),
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
            "text",
            "x",
            "y",
            "scale",
            "foreground",
            "background",
            "h_align",
            "v_align",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::String2D::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::String2DRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::String2D::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
