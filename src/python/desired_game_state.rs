use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};
use planus::{Builder, ReadAsRoot};
use pyo3::{prelude::*, types::*};

#[pyclass(module = "rlbot_flatbuffers", subclass, get_all)]
pub struct DesiredGameState {
    #[pyo3(set)]
    pub ball_states: Py<PyList>,
    #[pyo3(set)]
    pub car_states: Py<PyList>,
    #[pyo3(set)]
    pub match_info: Option<Py<super::DesiredMatchInfo>>,
    #[pyo3(set)]
    pub console_commands: Py<PyList>,
}

impl crate::PyDefault for DesiredGameState {
    fn py_default(py: Python) -> Py<Self> {
        Py::new(
            py,
            Self {
                ball_states: PyList::empty(py).unbind(),
                car_states: PyList::empty(py).unbind(),
                match_info: None,
                console_commands: PyList::empty(py).unbind(),
            },
        )
        .unwrap()
    }
}

impl FromGil<&flat::DesiredGameState> for DesiredGameState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, flat_t: &flat::DesiredGameState) -> Self {
        DesiredGameState {
            ball_states: PyList::new(
                py,
                flat_t
                    .ball_states
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::DesiredBallState>(py, x)),
            )
            .unwrap()
            .unbind(),
            car_states: PyList::new(
                py,
                flat_t
                    .car_states
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::DesiredCarState>(py, x)),
            )
            .unwrap()
            .unbind(),
            match_info: flat_t
                .match_info
                .as_ref()
                .map(|x| crate::into_py_from(py, &**x)),
            console_commands: PyList::new(
                py,
                flat_t
                    .console_commands
                    .iter()
                    .map(|x| crate::into_py_from::<_, super::ConsoleCommand>(py, x)),
            )
            .unwrap()
            .unbind(),
        }
    }
}

impl FromGil<&DesiredGameState> for flat::DesiredGameState {
    #[allow(unused_variables)]
    fn from_gil(py: Python, py_type: &DesiredGameState) -> Self {
        Self {
            ball_states: py_type
                .ball_states
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            car_states: py_type
                .car_states
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
            match_info: py_type
                .match_info
                .as_ref()
                .map(|x| Box::new(crate::from_py_into(py, x))),
            console_commands: py_type
                .console_commands
                .bind_borrowed(py)
                .iter()
                .map(|x| crate::from_pyany_into(py, x))
                .collect(),
        }
    }
}

#[pymethods]
impl DesiredGameState {
    #[new]
    #[pyo3(signature = (ball_states=None, car_states=None, match_info=None, console_commands=None))]
    pub fn new(
        py: Python,
        ball_states: Option<Py<PyList>>,
        car_states: Option<Py<PyList>>,
        match_info: Option<Py<super::DesiredMatchInfo>>,
        console_commands: Option<Py<PyList>>,
    ) -> Self {
        Self {
            ball_states: ball_states.unwrap_or_else(|| PyList::empty(py).unbind()),
            car_states: car_states.unwrap_or_else(|| PyList::empty(py).unbind()),
            match_info,
            console_commands: console_commands.unwrap_or_else(|| PyList::empty(py).unbind()),
        }
    }

    pub fn __str__(&self, py: Python) -> String {
        self.__repr__(py)
    }

    #[allow(unused_variables)]
    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "DesiredGameState(ball_states=[{}], car_states=[{}], match_info={}, console_commands=[{}])",
            self.ball_states
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::DesiredBallState>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.car_states
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::DesiredCarState>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
            self.match_info
                .as_ref()
                .map_or_else(crate::none_str, |x| { x.borrow(py).__repr__(py) }),
            self.console_commands
                .bind_borrowed(py)
                .iter()
                .map(|x| x
                    .cast_into::<super::ConsoleCommand>()
                    .unwrap()
                    .borrow()
                    .__repr__(py))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        (
            "ball_states",
            "car_states",
            "match_info",
            "console_commands",
        )
    }

    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut builder = Builder::with_capacity(u16::MAX as usize);

        let flat_t = flat::DesiredGameState::from_gil(py, self);
        PyBytes::new(py, builder.finish(flat_t, None))
    }

    #[staticmethod]
    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {
        let flat_t_ref = flat::DesiredGameStateRef::read_as_root(data).map_err(flat_err_to_py)?;
        let flat_t = flat::DesiredGameState::try_from(flat_t_ref).map_err(flat_err_to_py)?;

        Ok(crate::into_py_from(py, &flat_t))
    }
}
