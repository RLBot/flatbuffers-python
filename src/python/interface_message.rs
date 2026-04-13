use crate::{FromGil, PyDefault, flat};
use pyo3::prelude::*;

#[derive(pyo3::FromPyObject)]
pub enum InterfaceMessage {
    DisconnectSignal(Py<super::DisconnectSignal>),
    StartCommand(Py<super::StartCommand>),
    MatchConfiguration(Py<super::MatchConfiguration>),
    PlayerInput(Py<super::PlayerInput>),
    DesiredGameState(Py<super::DesiredGameState>),
    RenderGroup(Py<super::RenderGroup>),
    RemoveRenderGroup(Py<super::RemoveRenderGroup>),
    MatchComm(Py<super::MatchComm>),
    ConnectionSettings(Py<super::ConnectionSettings>),
    StopCommand(Py<super::StopCommand>),
    SetLoadout(Py<super::SetLoadout>),
    InitComplete(Py<super::InitComplete>),
    RenderingStatus(Py<super::RenderingStatus>),
    PingRequest(Py<super::PingRequest>),
    PingResponse(Py<super::PingResponse>),
}

impl InterfaceMessage {
    pub fn py_default(py: Python) -> Py<PyAny> {
        super::DisconnectSignal::py_default(py).into_any()
    }
}

impl FromGil<&flat::InterfaceMessage> for InterfaceMessage {
    fn from_gil(py: Python, flat_t: &flat::InterfaceMessage) -> Self {
        match flat_t {
            flat::InterfaceMessage::DisconnectSignal(item) => Self::DisconnectSignal(
                Py::new(py, super::DisconnectSignal::from_gil(py, &**item)).unwrap(),
            ),
            flat::InterfaceMessage::StartCommand(item) => {
                Self::StartCommand(Py::new(py, super::StartCommand::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::MatchConfiguration(item) => Self::MatchConfiguration(
                Py::new(py, super::MatchConfiguration::from_gil(py, &**item)).unwrap(),
            ),
            flat::InterfaceMessage::PlayerInput(item) => {
                Self::PlayerInput(Py::new(py, super::PlayerInput::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::DesiredGameState(item) => Self::DesiredGameState(
                Py::new(py, super::DesiredGameState::from_gil(py, &**item)).unwrap(),
            ),
            flat::InterfaceMessage::RenderGroup(item) => {
                Self::RenderGroup(Py::new(py, super::RenderGroup::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::RemoveRenderGroup(item) => Self::RemoveRenderGroup(
                Py::new(py, super::RemoveRenderGroup::from_gil(py, &**item)).unwrap(),
            ),
            flat::InterfaceMessage::MatchComm(item) => {
                Self::MatchComm(Py::new(py, super::MatchComm::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::ConnectionSettings(item) => Self::ConnectionSettings(
                Py::new(py, super::ConnectionSettings::from_gil(py, &**item)).unwrap(),
            ),
            flat::InterfaceMessage::StopCommand(item) => {
                Self::StopCommand(Py::new(py, super::StopCommand::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::SetLoadout(item) => {
                Self::SetLoadout(Py::new(py, super::SetLoadout::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::InitComplete(item) => {
                Self::InitComplete(Py::new(py, super::InitComplete::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::RenderingStatus(item) => Self::RenderingStatus(
                Py::new(py, super::RenderingStatus::from_gil(py, &**item)).unwrap(),
            ),
            flat::InterfaceMessage::PingRequest(item) => {
                Self::PingRequest(Py::new(py, super::PingRequest::from_gil(py, &**item)).unwrap())
            }
            flat::InterfaceMessage::PingResponse(item) => {
                Self::PingResponse(Py::new(py, super::PingResponse::from_gil(py, &**item)).unwrap())
            }
        }
    }
}

impl FromGil<&InterfaceMessage> for flat::InterfaceMessage {
    fn from_gil(py: Python, py_type: &InterfaceMessage) -> Self {
        match py_type {
            InterfaceMessage::DisconnectSignal(item) => {
                flat::InterfaceMessage::DisconnectSignal(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::StartCommand(item) => {
                flat::InterfaceMessage::StartCommand(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::MatchConfiguration(item) => {
                flat::InterfaceMessage::MatchConfiguration(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::PlayerInput(item) => {
                flat::InterfaceMessage::PlayerInput(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::DesiredGameState(item) => {
                flat::InterfaceMessage::DesiredGameState(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::RenderGroup(item) => {
                flat::InterfaceMessage::RenderGroup(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::RemoveRenderGroup(item) => {
                flat::InterfaceMessage::RemoveRenderGroup(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::MatchComm(item) => {
                flat::InterfaceMessage::MatchComm(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::ConnectionSettings(item) => {
                flat::InterfaceMessage::ConnectionSettings(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::StopCommand(item) => {
                flat::InterfaceMessage::StopCommand(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::SetLoadout(item) => {
                flat::InterfaceMessage::SetLoadout(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::InitComplete(item) => {
                flat::InterfaceMessage::InitComplete(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::RenderingStatus(item) => {
                flat::InterfaceMessage::RenderingStatus(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::PingRequest(item) => {
                flat::InterfaceMessage::PingRequest(Box::new(crate::from_py_into(py, item)))
            }
            InterfaceMessage::PingResponse(item) => {
                flat::InterfaceMessage::PingResponse(Box::new(crate::from_py_into(py, item)))
            }
        }
    }
}

impl InterfaceMessage {
    pub fn into_any(self) -> Py<PyAny> {
        match self {
            Self::DisconnectSignal(item) => item.into_any(),
            Self::StartCommand(item) => item.into_any(),
            Self::MatchConfiguration(item) => item.into_any(),
            Self::PlayerInput(item) => item.into_any(),
            Self::DesiredGameState(item) => item.into_any(),
            Self::RenderGroup(item) => item.into_any(),
            Self::RemoveRenderGroup(item) => item.into_any(),
            Self::MatchComm(item) => item.into_any(),
            Self::ConnectionSettings(item) => item.into_any(),
            Self::StopCommand(item) => item.into_any(),
            Self::SetLoadout(item) => item.into_any(),
            Self::InitComplete(item) => item.into_any(),
            Self::RenderingStatus(item) => item.into_any(),
            Self::PingRequest(item) => item.into_any(),
            Self::PingResponse(item) => item.into_any(),
        }
    }

    pub fn __repr__(&self, py: Python) -> String {
        match self {
            Self::DisconnectSignal(item) => item.borrow(py).__repr__(py),
            Self::StartCommand(item) => item.borrow(py).__repr__(py),
            Self::MatchConfiguration(item) => item.borrow(py).__repr__(py),
            Self::PlayerInput(item) => item.borrow(py).__repr__(py),
            Self::DesiredGameState(item) => item.borrow(py).__repr__(py),
            Self::RenderGroup(item) => item.borrow(py).__repr__(py),
            Self::RemoveRenderGroup(item) => item.borrow(py).__repr__(py),
            Self::MatchComm(item) => item.borrow(py).__repr__(py),
            Self::ConnectionSettings(item) => item.borrow(py).__repr__(py),
            Self::StopCommand(item) => item.borrow(py).__repr__(py),
            Self::SetLoadout(item) => item.borrow(py).__repr__(py),
            Self::InitComplete(item) => item.borrow(py).__repr__(py),
            Self::RenderingStatus(item) => item.borrow(py).__repr__(py),
            Self::PingRequest(item) => item.borrow(py).__repr__(py),
            Self::PingResponse(item) => item.borrow(py).__repr__(py),
        }
    }
}
