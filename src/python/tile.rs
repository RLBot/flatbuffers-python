use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct Tile {
    #[pyo3(set)]
    pub location: Py<super::Vector3>,
    #[pyo3(set)]
    pub team: u32,
}

impl crate::PyDefault for Tile {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                location: super::Vector3::py_default(py),
                team: Default::default(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::Tile> for Tile {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::Tile) -> Self {
        Tile {
            location: crate::into_py_from(py, &flat_t.location),
            team: flat_t.team,
        }
    }
}

impl FromGil<&Tile> for flat::Tile {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &Tile) -> Self {
        Self {
            location: crate::from_py_into(py, &py_type.location),
            team: py_type.team,
        }
    }
}

#[pymethods]
impl Tile {
    #[new]
    #[pyo3(signature = (location=None, team=0))]
    pub fn new(py: Python, location: Option<Py<super::Vector3>>, team: u32) -> Self {
        Self {
            location: location.unwrap_or_else(|| super::Vector3::py_default(py)),
            team,
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "Tile(location={}, team={})",
            self.location.borrow(py).__repr__(py),
            self.team,
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("location", "team")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::Tile::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::TileRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::Tile::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
