use indexmap::IndexMap;
use planus_types::intermediate::UnionVariant;
use std::borrow::Cow;

pub const NO_SET_TYPES: [&str; 1] = ["PlayerClass"];

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
    is_no_set: bool,
}

impl<'a> UnionBindGenerator<'a> {
    pub fn new(name: &'a str, variants: &'a IndexMap<String, UnionVariant>) -> Self {
        Self {
            name,
            variants,
            file_contents: Vec::new(),
            is_no_set: NO_SET_TYPES.contains(&name),
        }
    }

    fn generate_definition(&mut self) {
        write_fmt!(self, "#[derive(pyo3::FromPyObject)]");
        write_fmt!(self, "pub enum {}Union {{", self.name);

        for var_name in self.variants.keys() {
            write_fmt!(self, "    {var_name}(Py<super::{var_name}>),");
        }

        write_str!(self, "}");
        write_str!(self, "");

        if self.is_no_set {
            write_str!(self, "#[pyclass(module = \"rlbot_flatbuffers\")]");
        } else {
            write_str!(self, "#[pyclass(module = \"rlbot_flatbuffers\", set_all)]");
        }

        write_fmt!(self, "pub struct {} {{", self.name);
        write_fmt!(self, "    item: {}Union,", self.name);
        write_str!(self, "}");
        write_str!(self, "");

        write_fmt!(self, "impl crate::PyDefault for {} {{", self.name);
        write_str!(self, "    fn py_default(py: Python) -> Py<Self> {");
        write_str!(self, "        Py::new(py, Self {");

        let first_var_name = self.variants.keys().next().unwrap();
        write_fmt!(
            self,
            "            item: {}Union::{first_var_name}(super::{first_var_name}::py_default(py)),",
            self.name
        );
        write_str!(self, "        }).unwrap()");
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
                "            flat::{}::{var_name}(item) => {} {{",
                self.name,
                self.name,
            );

            write_fmt!(
                self,
                "                item: {}Union::{var_name}(",
                self.name
            );

            write_fmt!(
                self,
                "                    Py::new(py, super::{var_name}::from_gil(py, *item)).unwrap(),"
            );

            write_fmt!(self, "                ),");
            write_fmt!(self, "            }},");
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

        write_str!(self, "        match &py_type.item {");

        for var_name in self.variants.keys() {
            write_fmt!(
                self,
                "            {}Union::{var_name}(item) => {{",
                self.name,
            );

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

    fn generate_new_method(&mut self) {
        assert!(u8::try_from(self.variants.len()).is_ok());

        write_str!(self, "    #[new]");
        write_fmt!(self, "    pub fn new(item: {}Union) -> Self {{", self.name);
        write_str!(self, "        Self { item }");
        write_str!(self, "    }");
        write_str!(self, "");
        write_str!(self, "    #[getter(item)]");

        write_str!(
            self,
            "    pub fn get(&self, py: Python) -> Option<Py<PyAny>> {"
        );
        write_str!(self, "        match &self.item {");

        for var_name in self.variants.keys() {
            write_fmt!(
                self,
                "            {}Union::{var_name}(item) => Some(item.clone_ref(py).into_any()),",
                self.name
            );
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
    }

    fn generate_str_method(&mut self) {
        write_str!(self, "    pub fn __str__(&self, py: Python) -> String {");
        write_str!(self, "        self.__repr__(py)");
        write_str!(self, "    }");
    }

    fn generate_inner_repr_method(&mut self) {
        write_str!(self, "    pub fn inner_repr(&self, py: Python) -> String {");
        write_str!(self, "        match &self.item {");

        for var_name in self.variants.keys() {
            write_fmt!(
                self,
                "            {}Union::{var_name}(item) => item.borrow(py).__repr__(py),",
                self.name
            );
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
    }

    fn generate_repr_method(&mut self) {
        write_str!(self, "    pub fn __repr__(&self, py: Python) -> String {");
        write_str!(self, "        match &self.item {");

        for var_name in self.variants.keys() {
            write_fmt!(
                self,
                "            {}Union::{var_name}(item) => format!(\"{}({{}})\", item.borrow(py).__repr__(py)),",
                self.name,
                self.name
            );
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
    }

    fn generate_py_methods(&mut self) {
        write_str!(self, "#[pymethods]");
        write_fmt!(self, "impl {} {{", self.name);

        self.generate_new_method();
        write_str!(self, "");

        self.generate_str_method();
        write_str!(self, "");

        self.generate_inner_repr_method();
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
