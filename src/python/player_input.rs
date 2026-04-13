use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, frozen, get_all)]
pub struct PlayerInput {
    pub player_index: u32,
    pub controller_state: Py<super::ControllerState>,
}

impl crate::PyDefault for PlayerInput {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                player_index: Default::default(),
                controller_state: super::ControllerState::py_default(py),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::PlayerInput> for PlayerInput {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::PlayerInput) -> Self {
        PlayerInput {
            player_index: flat_t.player_index,
            controller_state: crate::into_py_from(py, &flat_t.controller_state),
        }
    }
}

impl FromGil<&PlayerInput> for flat::PlayerInput {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &PlayerInput) -> Self {
        Self {
            player_index: py_type.player_index,
            controller_state: crate::from_py_into(py, &py_type.controller_state),
        }
    }
}

#[pymethods]
impl PlayerInput {
    #[new]
    #[pyo3(signature = (player_index=0, controller_state=None))]
    pub fn new(
        py: Python,
        player_index: u32,
        controller_state: Option<Py<super::ControllerState>>,
    ) -> Self {
        Self {
            player_index,
            controller_state: controller_state
                .unwrap_or_else(|| super::ControllerState::py_default(py)),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "PlayerInput(player_index={}, controller_state={})",
            self.player_index,
            self.controller_state.borrow(py).__repr__(py),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("player_index", "controller_state")
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::PlayerInput::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::PlayerInputRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::PlayerInput::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
