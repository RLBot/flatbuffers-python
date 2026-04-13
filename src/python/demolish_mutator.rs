use crate::flat;
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[pyclass(module = "rlbot_flatbuffers", from_py_object, frozen, hash, eq, eq_int)]
pub enum DemolishMutator {
    #[default]
    Default = 0,
    Disabled = 1,
    FriendlyFire = 2,
    OnContact = 3,
    OnContactFf = 4,
    OnBallContact = 5,
    OnBallContactFf = 6,
    OnBallContactSilent = 7,
    OnBallContactFfSilent = 8,
}

impl From<flat::DemolishMutator> for DemolishMutator {
    fn from(flat_t: flat::DemolishMutator) -> Self {
        match flat_t {
            flat::DemolishMutator::Default => Self::Default,
            flat::DemolishMutator::Disabled => Self::Disabled,
            flat::DemolishMutator::FriendlyFire => Self::FriendlyFire,
            flat::DemolishMutator::OnContact => Self::OnContact,
            flat::DemolishMutator::OnContactFf => Self::OnContactFf,
            flat::DemolishMutator::OnBallContact => Self::OnBallContact,
            flat::DemolishMutator::OnBallContactFf => Self::OnBallContactFf,
            flat::DemolishMutator::OnBallContactSilent => Self::OnBallContactSilent,
            flat::DemolishMutator::OnBallContactFfSilent => Self::OnBallContactFfSilent,
        }
    }
}

impl From<&DemolishMutator> for flat::DemolishMutator {
    fn from(py_type: &DemolishMutator) -> Self {
        match py_type {
            DemolishMutator::Default => Self::Default,
            DemolishMutator::Disabled => Self::Disabled,
            DemolishMutator::FriendlyFire => Self::FriendlyFire,
            DemolishMutator::OnContact => Self::OnContact,
            DemolishMutator::OnContactFf => Self::OnContactFf,
            DemolishMutator::OnBallContact => Self::OnBallContact,
            DemolishMutator::OnBallContactFf => Self::OnBallContactFf,
            DemolishMutator::OnBallContactSilent => Self::OnBallContactSilent,
            DemolishMutator::OnBallContactFfSilent => Self::OnBallContactFfSilent,
        }
    }
}

#[pymethods]
impl DemolishMutator {
    #[new]
    #[pyo3(signature = (value=Default::default()))]
    pub fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Disabled),
            2 => Ok(Self::FriendlyFire),
            3 => Ok(Self::OnContact),
            4 => Ok(Self::OnContactFf),
            5 => Ok(Self::OnBallContact),
            6 => Ok(Self::OnBallContactFf),
            7 => Ok(Self::OnBallContactSilent),
            8 => Ok(Self::OnBallContactFfSilent),
            v => Err(PyValueError::new_err(format!("Unknown value of {v}"))),
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }

    pub fn __repr__(&self) -> String {
        format!("DemolishMutator.{self:?}")
    }
}
