use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

pub use flat::RumbleMutator;

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
