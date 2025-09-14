use indexmap::IndexMap;
use planus_types::intermediate::UnionVariant;
use std::borrow::Cow;

macro_rules! write_str {
    ($self:ident, $s:expr) => {
        $self.file_contents.push(Cow::Borrowed($s))
    };
}

macro_rules! write_fmt {
    ($self:ident, $($arg:tt)*) => {
        $self.file_contents.push(Cow::Owned(format!($($arg)*)))
    };
}

pub struct UnionBindGenerator<'a> {
    name: &'a str,
    variants: &'a IndexMap<String, UnionVariant>,
    file_contents: Vec<Cow<'static, str>>,
}

impl<'a> UnionBindGenerator<'a> {
    pub fn new(name: &'a str, variants: &'a IndexMap<String, UnionVariant>) -> Self {
        Self {
            name,
            variants,
            file_contents: Vec::new(),
        }
    }

    fn generate_definition(&mut self) {
        write_fmt!(self, "#[derive(pyo3::FromPyObject)]");
        write_fmt!(self, "pub enum {} {{", self.name);

        for var_name in self.variants.keys() {
            write_fmt!(self, "    {var_name}(Py<super::{var_name}>),");
        }

        write_str!(self, "}");
        write_str!(self, "");

        write_fmt!(self, "impl {} {{", self.name);
        write_str!(self, "    pub fn py_default(py: Python) -> Py<PyAny> {");

        let first_var_name = self.variants.keys().next().unwrap();
        write_fmt!(
            self,
            "            super::{first_var_name}::py_default(py).into_any()",
        );
        write_str!(self, "    }");
        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_from_flat_impls(&mut self) {
        write_fmt!(
            self,
            "impl FromGil<flat::{}> for {} {{",
            self.name,
            self.name
        );
        write_fmt!(
            self,
            "    fn from_gil(py: Python, flat_t: flat::{}) -> Self {{",
            self.name
        );

        write_str!(self, "        match flat_t {");

        for var_name in self.variants.keys() {
            write_fmt!(
                self,
                "            flat::{}::{var_name}(item) => ",
                self.name,
            );
            write_fmt!(self, "                Self::{var_name}(");

            write_fmt!(
                self,
                "                    Py::new(py, super::{var_name}::from_gil(py, *item)).unwrap(),"
            );

            write_str!(self, "                ),");
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_to_flat_impls(&mut self) {
        write_fmt!(
            self,
            "impl FromGil<&{}> for flat::{} {{",
            self.name,
            self.name
        );
        write_fmt!(
            self,
            "    fn from_gil(py: Python, py_type: &{}) -> Self {{",
            self.name
        );

        write_str!(self, "        match py_type {");

        for var_name in self.variants.keys() {
            write_fmt!(self, "            {}::{var_name}(item) => {{", self.name,);

            write_fmt!(
                self,
                "                flat::{}::{var_name}(Box::new(crate::from_py_into(py, item)))",
                self.name
            );

            write_str!(self, "            },");
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_into_pyany_method(&mut self) {
        write_str!(self, "    pub fn into_any(self) -> Py<PyAny> {");
        write_str!(self, "        match self {");

        for var_name in self.variants.keys() {
            write_fmt!(
                self,
                "            Self::{var_name}(item) => item.into_any(),"
            );
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
    }

    fn generate_repr_method(&mut self) {
        write_str!(self, "    pub fn __repr__(&self, py: Python) -> String {");
        write_str!(self, "        match self {");

        for var_name in self.variants.keys() {
            write_fmt!(
                self,
                "            Self::{var_name}(item) => item.borrow(py).__repr__(py),"
            );
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
    }

    fn generate_py_methods(&mut self) {
        write_fmt!(self, "impl {} {{", self.name);

        self.generate_into_pyany_method();
        write_str!(self, "");

        self.generate_repr_method();
        write_str!(self, "}");
        write_str!(self, "");
    }

    pub fn generate_binds(mut self) -> Vec<Cow<'static, str>> {
        write_str!(self, "use crate::{FromGil, PyDefault, flat};");
        write_str!(self, "use pyo3::prelude::*;");
        write_str!(self, "");

        self.generate_definition();
        self.generate_from_flat_impls();
        self.generate_to_flat_impls();
        self.generate_py_methods();

        self.file_contents
    }
}
