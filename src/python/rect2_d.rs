use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct Rect2D {
    pub x: Py<PyFloat>,
    pub y: Py<PyFloat>,
    pub width: Py<PyFloat>,
    pub height: Py<PyFloat>,
    #[pyo3(set)]
    pub color: Py<super::Color>,
    #[pyo3(set)]
    pub h_align: super::TextHAlign,
    #[pyo3(set)]
    pub v_align: super::TextVAlign,
}

impl crate::PyDefault for Rect2D {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                x: crate::pyfloat_default(py),
                y: crate::pyfloat_default(py),
                width: crate::pyfloat_default(py),
                height: crate::pyfloat_default(py),
                color: super::Color::py_default(py),
                h_align: Default::default(),
                v_align: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Rect2D> for Rect2D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Rect2D) -> Self {
        Rect2D {
            x: crate::float_to_py(py, flat_t.x),
            y: crate::float_to_py(py, flat_t.y),
            width: crate::float_to_py(py, flat_t.width),
            height: crate::float_to_py(py, flat_t.height),
            color: crate::into_py_from(py, &flat_t.color),
            h_align: flat_t.h_align.into(),
            v_align: flat_t.v_align.into(),
        }
    }
}

impl FromGil<&Rect2D> for flat::Rect2D {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Rect2D) -> Self {
        Self {
            x: crate::float_from_py(py, &py_type.x),
            y: crate::float_from_py(py, &py_type.y),
            width: crate::float_from_py(py, &py_type.width),
            height: crate::float_from_py(py, &py_type.height),
            color: crate::from_py_into(py, &py_type.color),
            h_align: (&py_type.h_align).into(),
            v_align: (&py_type.v_align).into(),
        }
    }
}

#[pymethods]
impl Rect2D {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (x=0.0, y=0.0, width=0.0, height=0.0, color=None, h_align=Default::default(), v_align=Default::default()))]
    pub fn new(
        py: Python,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: Option<Py<super::Color>>,
        h_align: super::TextHAlign,
        v_align: super::TextVAlign,
    ) -> Self {
        Self {
            x: PyFloat::new(py, x).unbind(),
            y: PyFloat::new(py, y).unbind(),
            width: PyFloat::new(py, width).unbind(),
            height: PyFloat::new(py, height).unbind(),
            color: color.unwrap_or_else(|| super::Color::py_default(py)),
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
    pub fn width(&mut self, py: Python, value: f64) {
        self.width = PyFloat::new(py, value).unbind();
    }

    #[setter]
    pub fn height(&mut self, py: Python, value: f64) {
        self.height = PyFloat::new(py, value).unbind();
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Rect2D(x={}, y={}, width={}, height={}, color={}, h_align={}, v_align={})",
            self.x,
            self.y,
            self.width,
            self.height,
            self.color.borrow(py).__repr__(py),
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
        ("x", "y", "width", "height", "color", "h_align", "v_align")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Rect2D::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::Rect2DRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Rect2D::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
