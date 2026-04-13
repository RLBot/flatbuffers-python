use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum AirState {
    #[default]
    OnGround = 0,
    Jumping = 1,
    DoubleJumping = 2,
    Dodging = 3,
    InAir = 4,
}

impl From<flat::AirState> for AirState {
    fn from(flat_t: flat::AirState) -> Self {
        match flat_t {
            flat::AirState::OnGround => Self::OnGround,
            flat::AirState::Jumping => Self::Jumping,
            flat::AirState::DoubleJumping => Self::DoubleJumping,
            flat::AirState::Dodging => Self::Dodging,
            flat::AirState::InAir => Self::InAir,
        }
    }
}

impl From<&AirState> for flat::AirState {
    fn from(py_type: &AirState) -> Self {
        match py_type {
            AirState::OnGround => Self::OnGround,
            AirState::Jumping => Self::Jumping,
            AirState::DoubleJumping => Self::DoubleJumping,
            AirState::Dodging => Self::Dodging,
            AirState::InAir => Self::InAir,
        }
    }
}

#[pymethods]
impl AirState {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::OnGround),
            1 => Ok(Self::Jumping),
            2 => Ok(Self::DoubleJumping),
            3 => Ok(Self::Dodging),
            4 => Ok(Self::InAir),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("AirState.{self:?}")
    }
}
