use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum PsyonixSkill {
    #[default]
    Beginner = 0,
    Rookie = 1,
    Pro = 2,
    AllStar = 3,
}

impl From<flat::PsyonixSkill> for PsyonixSkill {
    fn from(flat_t: flat::PsyonixSkill) -> Self {
        match flat_t {
            flat::PsyonixSkill::Beginner => Self::Beginner,
            flat::PsyonixSkill::Rookie => Self::Rookie,
            flat::PsyonixSkill::Pro => Self::Pro,
            flat::PsyonixSkill::AllStar => Self::AllStar,
        }
    }
}

impl From<&PsyonixSkill> for flat::PsyonixSkill {
    fn from(py_type: &PsyonixSkill) -> Self {
        match py_type {
            PsyonixSkill::Beginner => Self::Beginner,
            PsyonixSkill::Rookie => Self::Rookie,
            PsyonixSkill::Pro => Self::Pro,
            PsyonixSkill::AllStar => Self::AllStar,
        }
    }
}

#[pymethods]
impl PsyonixSkill {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Beginner),
            1 => Ok(Self::Rookie),
            2 => Ok(Self::Pro),
            3 => Ok(Self::AllStar),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("PsyonixSkill.{self:?}")
    }
}
