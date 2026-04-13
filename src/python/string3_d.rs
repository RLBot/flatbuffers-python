use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct String3D {
    #[pyo3(set)]
    pub text: Py<PyString>,
    #[pyo3(set)]
    pub anchor: Py<super::RenderAnchor>,
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

impl crate::PyDefault for String3D {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                text: crate::pydefault_string(py),
                anchor: super::RenderAnchor::py_default(py),
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

impl FromGil<&flat::String3D> for String3D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::String3D) -> Self {
        String3D {
            text: PyString::new(py, &flat_t.text).unbind(),
            anchor: crate::into_py_from(py, &*flat_t.anchor),
            scale: crate::float_to_py(py, flat_t.scale),
            foreground: crate::into_py_from(py, &flat_t.foreground),
            background: crate::into_py_from(py, &flat_t.background),
            h_align: flat_t.h_align,
            v_align: flat_t.v_align,
        }
    }
}

impl FromGil<&String3D> for flat::String3D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &String3D) -> Self {
        Self {
            text: py_type.text.to_str(py).unwrap().to_string(),
            anchor: Box::new(crate::from_py_into(py, &py_type.anchor)),
            scale: crate::float_from_py(py, &py_type.scale),
            foreground: crate::from_py_into(py, &py_type.foreground),
            background: crate::from_py_into(py, &py_type.background),
            h_align: py_type.h_align,
            v_align: py_type.v_align,
        }
    }
}

#[pymethods]
impl String3D {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (text=None, anchor=None, scale=0.0, foreground=None, background=None, h_align=Default::default(), v_align=Default::default()))]
    pub fn new(
        py: Python,
        text: Option<Py<PyString>>,
        anchor: Option<Py<super::RenderAnchor>>,
        scale: f64,
        foreground: Option<Py<super::Color>>,
        background: Option<Py<super::Color>>,
        h_align: super::TextHAlign,
        v_align: super::TextVAlign,
    ) -> Self {
        Self {
            text: text.unwrap_or_else(|| crate::pydefault_string(py)),
            anchor: anchor.unwrap_or_else(|| super::RenderAnchor::py_default(py)),
            scale: PyFloat::new(py, scale).unbind(),
            foreground: foreground.unwrap_or_else(|| super::Color::py_default(py)),
            background: background.unwrap_or_else(|| super::Color::py_default(py)),
            h_align,
            v_align,
        }
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
            "String3D(text={:?}, anchor={}, scale={}, foreground={}, background={}, h_align={}, v_align={})",
            self.text.bind(py).to_cow().unwrap(),
            self.anchor.borrow(py).__repr__(py),
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
    ) {
        (
            "text",
            "anchor",
            "scale",
            "foreground",
            "background",
            "h_align",
            "v_align",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::String3D::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::String3DRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::String3D::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
