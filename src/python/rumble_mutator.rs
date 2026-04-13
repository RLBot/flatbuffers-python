use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum RumbleMutator {
    #[default]
    Off = 0,
    DefaultRumble = 1,
    Slow = 2,
    Civilized = 3,
    DestructionDerby = 4,
    SpringLoaded = 5,
    SpikesOnly = 6,
    SpikeRush = 7,
    HauntedBallBeam = 8,
    Tactical = 9,
    BatmanRumble = 10,
    GrapplingOnly = 11,
    HaymakerOnly = 12,
    SpikeRushForce = 13,
    Rps = 14,
}

impl From<flat::RumbleMutator> for RumbleMutator {
    fn from(flat_t: flat::RumbleMutator) -> Self {
        match flat_t {
            flat::RumbleMutator::Off => Self::Off,
            flat::RumbleMutator::DefaultRumble => Self::DefaultRumble,
            flat::RumbleMutator::Slow => Self::Slow,
            flat::RumbleMutator::Civilized => Self::Civilized,
            flat::RumbleMutator::DestructionDerby => Self::DestructionDerby,
            flat::RumbleMutator::SpringLoaded => Self::SpringLoaded,
            flat::RumbleMutator::SpikesOnly => Self::SpikesOnly,
            flat::RumbleMutator::SpikeRush => Self::SpikeRush,
            flat::RumbleMutator::HauntedBallBeam => Self::HauntedBallBeam,
            flat::RumbleMutator::Tactical => Self::Tactical,
            flat::RumbleMutator::BatmanRumble => Self::BatmanRumble,
            flat::RumbleMutator::GrapplingOnly => Self::GrapplingOnly,
            flat::RumbleMutator::HaymakerOnly => Self::HaymakerOnly,
            flat::RumbleMutator::SpikeRushForce => Self::SpikeRushForce,
            flat::RumbleMutator::Rps => Self::Rps,
        }
    }
}

impl From<&RumbleMutator> for flat::RumbleMutator {
    fn from(py_type: &RumbleMutator) -> Self {
        match py_type {
            RumbleMutator::Off => Self::Off,
            RumbleMutator::DefaultRumble => Self::DefaultRumble,
            RumbleMutator::Slow => Self::Slow,
            RumbleMutator::Civilized => Self::Civilized,
            RumbleMutator::DestructionDerby => Self::DestructionDerby,
            RumbleMutator::SpringLoaded => Self::SpringLoaded,
            RumbleMutator::SpikesOnly => Self::SpikesOnly,
            RumbleMutator::SpikeRush => Self::SpikeRush,
            RumbleMutator::HauntedBallBeam => Self::HauntedBallBeam,
            RumbleMutator::Tactical => Self::Tactical,
            RumbleMutator::BatmanRumble => Self::BatmanRumble,
            RumbleMutator::GrapplingOnly => Self::GrapplingOnly,
            RumbleMutator::HaymakerOnly => Self::HaymakerOnly,
            RumbleMutator::SpikeRushForce => Self::SpikeRushForce,
            RumbleMutator::Rps => Self::Rps,
        }
    }
}

#[pymethods]
impl RumbleMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::DefaultRumble),
            2 => Ok(Self::Slow),
            3 => Ok(Self::Civilized),
            4 => Ok(Self::DestructionDerby),
            5 => Ok(Self::SpringLoaded),
            6 => Ok(Self::SpikesOnly),
            7 => Ok(Self::SpikeRush),
            8 => Ok(Self::HauntedBallBeam),
            9 => Ok(Self::Tactical),
            10 => Ok(Self::BatmanRumble),
            11 => Ok(Self::GrapplingOnly),
            12 => Ok(Self::HaymakerOnly),
            13 => Ok(Self::SpikeRushForce),
            14 => Ok(Self::Rps),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("RumbleMutator.{self:?}")
    }
}
