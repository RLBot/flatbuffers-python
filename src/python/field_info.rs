use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct FieldInfo {
    pub boost_pads: Py<PyList>,
    pub goals: Py<PyList>,
    pub tiles: Py<PyList>,
}

impl crate::PyDefault for FieldInfo {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                boost_pads: PyList::empty(py).unbind(),
                goals: PyList::empty(py).unbind(),
                tiles: PyList::empty(py).unbind(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::FieldInfo> for FieldInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::FieldInfo) -> Self {
        FieldInfo {
            boost_pads: PyList::new(
                py,
                flat_t
                    .boost_pads
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::BoostPad>(py, x)),
            )
            .unwrap()
            .unbind(),
            goals: PyList::new(
                py,
                flat_t
                    .goals
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::GoalInfo>(py, x)),
            )
            .unwrap()
            .unbind(),
            tiles: PyList::new(
                py,
                flat_t
                    .tiles
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::Tile>(py, x)),
            )
            .unwrap()
            .unbind(),
        }
    }
}

impl FromGil<&FieldInfo> for flat::FieldInfo {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &FieldInfo) -> Self {
        Self {
            boost_pads: py_type
                .boost_pads
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            goals: py_type
                .goals
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            tiles: py_type
                .tiles
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
        }
    }
}

#[pymethods]
impl FieldInfo {
    #[new]
    #[pyo3(signature = (boost_pads=None, goals=None, tiles=None))]
    pub fn new(
        py: Python,
        boost_pads: Option<Py<PyList>>,
        goals: Option<Py<PyList>>,
        tiles: Option<Py<PyList>>,
    ) -> Self {
        Self {
            boost_pads: boost_pads.unwrap_or_else(|| PyList::empty(py).unbind()),
            goals: goals.unwrap_or_else(|| PyList::empty(py).unbind()),
            tiles: tiles.unwrap_or_else(|| PyList::empty(py).unbind()),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "FieldInfo(boost_pads=[{}], goals=[{}], tiles=[{}])",
            self.boost_pads
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::BoostPad>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.goals
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::GoalInfo>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.tiles
                .bind_borrowed(py)
                .iter()
                .map(|x| x.cast_into::<super::Tile>().unwrap().borrow().__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("boost_pads", "goals", "tiles")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::FieldInfo::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::FieldInfoRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::FieldInfo::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
