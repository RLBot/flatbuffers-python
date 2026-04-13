use crate::{FromGil, PyDefault, flat};
use pyo3::prelude::*;

#[derive(pyo3::FromPyObject)]
pub enum CoreMessage {
    DisconnectSignal(Py<super::DisconnectSignal>),
    GamePacket(Py<super::GamePacket>),
    FieldInfo(Py<super::FieldInfo>),
    MatchConfiguration(Py<super::MatchConfiguration>),
    MatchComm(Py<super::MatchComm>),
    BallPrediction(Py<super::BallPrediction>),
    ControllableTeamInfo(Py<super::ControllableTeamInfo>),
    RenderingStatus(Py<super::RenderingStatus>),
    PingRequest(Py<super::PingRequest>),
    PingResponse(Py<super::PingResponse>),
}

impl CoreMessage {
    pub fn py_default(py: Python) -> Py<PyAny> {
        super::DisconnectSignal::py_default(py).into_any()
    }
}

impl FromGil<&flat::CoreMessage> for CoreMessage {
    fn from_gil(py: Python, flat_t: &flat::CoreMessage) -> Self {
        match flat_t {
            flat::CoreMessage::DisconnectSignal(item) => Self::DisconnectSignal(
                Py::new(py, super::DisconnectSignal::from_gil(py, &**item)).unwrap(),
            ),
            flat::CoreMessage::GamePacket(item) => {
                Self::GamePacket(Py::new(py, super::GamePacket::from_gil(py, &**item)).unwrap())
            }
            flat::CoreMessage::FieldInfo(item) => {
                Self::FieldInfo(Py::new(py, super::FieldInfo::from_gil(py, &**item)).unwrap())
            }
            flat::CoreMessage::MatchConfiguration(item) => Self::MatchConfiguration(
                Py::new(py, super::MatchConfiguration::from_gil(py, &**item)).unwrap(),
            ),
            flat::CoreMessage::MatchComm(item) => {
                Self::MatchComm(Py::new(py, super::MatchComm::from_gil(py, &**item)).unwrap())
            }
            flat::CoreMessage::BallPrediction(item) => Self::BallPrediction(
                Py::new(py, super::BallPrediction::from_gil(py, &**item)).unwrap(),
            ),
            flat::CoreMessage::ControllableTeamInfo(item) => Self::ControllableTeamInfo(
                Py::new(py, super::ControllableTeamInfo::from_gil(py, &**item)).unwrap(),
            ),
            flat::CoreMessage::RenderingStatus(item) => Self::RenderingStatus(
                Py::new(py, super::RenderingStatus::from_gil(py, &**item)).unwrap(),
            ),
            flat::CoreMessage::PingRequest(item) => {
                Self::PingRequest(Py::new(py, super::PingRequest::from_gil(py, &**item)).unwrap())
            }
            flat::CoreMessage::PingResponse(item) => {
                Self::PingResponse(Py::new(py, super::PingResponse::from_gil(py, &**item)).unwrap())
            }
        }
    }
}

impl FromGil<&CoreMessage> for flat::CoreMessage {
    fn from_gil(py: Python, py_type: &CoreMessage) -> Self {
        match py_type {
            CoreMessage::DisconnectSignal(item) => {
                flat::CoreMessage::DisconnectSignal(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::GamePacket(item) => {
                flat::CoreMessage::GamePacket(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::FieldInfo(item) => {
                flat::CoreMessage::FieldInfo(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::MatchConfiguration(item) => {
                flat::CoreMessage::MatchConfiguration(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::MatchComm(item) => {
                flat::CoreMessage::MatchComm(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::BallPrediction(item) => {
                flat::CoreMessage::BallPrediction(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::ControllableTeamInfo(item) => {
                flat::CoreMessage::ControllableTeamInfo(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::RenderingStatus(item) => {
                flat::CoreMessage::RenderingStatus(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::PingRequest(item) => {
                flat::CoreMessage::PingRequest(Box::new(crate::from_py_into(py, item)))
            }
            CoreMessage::PingResponse(item) => {
                flat::CoreMessage::PingResponse(Box::new(crate::from_py_into(py, item)))
            }
        }
    }
}

impl CoreMessage {
    pub fn into_any(self) -> Py<PyAny> {
        match self {
            Self::DisconnectSignal(item) => item.into_any(),
            Self::GamePacket(item) => item.into_any(),
            Self::FieldInfo(item) => item.into_any(),
            Self::MatchConfiguration(item) => item.into_any(),
            Self::MatchComm(item) => item.into_any(),
            Self::BallPrediction(item) => item.into_any(),
            Self::ControllableTeamInfo(item) => item.into_any(),
            Self::RenderingStatus(item) => item.into_any(),
            Self::PingRequest(item) => item.into_any(),
            Self::PingResponse(item) => item.into_any(),
        }
    }

    pub fn __repr__(&self, py: Python) -> String {
        match self {
            Self::DisconnectSignal(item) => item.borrow(py).__repr__(py),
            Self::GamePacket(item) => item.borrow(py).__repr__(py),
            Self::FieldInfo(item) => item.borrow(py).__repr__(py),
            Self::MatchConfiguration(item) => item.borrow(py).__repr__(py),
            Self::MatchComm(item) => item.borrow(py).__repr__(py),
            Self::BallPrediction(item) => item.borrow(py).__repr__(py),
            Self::ControllableTeamInfo(item) => item.borrow(py).__repr__(py),
            Self::RenderingStatus(item) => item.borrow(py).__repr__(py),
            Self::PingRequest(item) => item.borrow(py).__repr__(py),
            Self::PingResponse(item) => item.borrow(py).__repr__(py),
        }
    }
}
