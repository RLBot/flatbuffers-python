use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum GameMode {
    #[default]
    Soccar = 0,
    Hoops = 1,
    Dropshot = 2,
    Snowday = 3,
    Rumble = 4,
    Heatseeker = 5,
    Gridiron = 6,
    Knockout = 7,
}

impl From<flat::GameMode> for GameMode {
    fn from(flat_t: flat::GameMode) -> Self {
        match flat_t {
            flat::GameMode::Soccar => Self::Soccar,
            flat::GameMode::Hoops => Self::Hoops,
            flat::GameMode::Dropshot => Self::Dropshot,
            flat::GameMode::Snowday => Self::Snowday,
            flat::GameMode::Rumble => Self::Rumble,
            flat::GameMode::Heatseeker => Self::Heatseeker,
            flat::GameMode::Gridiron => Self::Gridiron,
            flat::GameMode::Knockout => Self::Knockout,
        }
    }
}

impl From<&GameMode> for flat::GameMode {
    fn from(py_type: &GameMode) -> Self {
        match py_type {
            GameMode::Soccar => Self::Soccar,
            GameMode::Hoops => Self::Hoops,
            GameMode::Dropshot => Self::Dropshot,
            GameMode::Snowday => Self::Snowday,
            GameMode::Rumble => Self::Rumble,
            GameMode::Heatseeker => Self::Heatseeker,
            GameMode::Gridiron => Self::Gridiron,
            GameMode::Knockout => Self::Knockout,
        }
    }
}

#[pymethods]
impl GameMode {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Soccar),
            1 => Ok(Self::Hoops),
            2 => Ok(Self::Dropshot),
            3 => Ok(Self::Snowday),
            4 => Ok(Self::Rumble),
            5 => Ok(Self::Heatseeker),
            6 => Ok(Self::Gridiron),
            7 => Ok(Self::Knockout),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("GameMode.{self:?}")
    }
}
