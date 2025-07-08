use crate::{FROZEN_TYPES, get_int_name};
use indexmap::IndexMap;
use planus_types::intermediate::{AbsolutePath, Declaration, SimpleType, StructField};
use std::{borrow::Cow, iter::repeat_n};

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

pub const DEFAULT_OVERRIDES: [(&str, &str, &str); 1] = [("Color", "a", "255")];

pub struct StructBindGenerator<'a> {
    name: &'a str,
    fields: &'a IndexMap<String, StructField>,
    all_items: &'a IndexMap<AbsolutePath, Declaration>,
    default_overrides: Vec<(&'a str, &'a str)>,
    file_contents: Vec<Cow<'static, str>>,
    is_frozen: bool,
}

impl<'a> StructBindGenerator<'a> {
    pub fn new(
        name: &'a str,
        fields: &'a IndexMap<String, StructField>,
        all_items: &'a IndexMap<AbsolutePath, Declaration>,
    ) -> Self {
        let default_overrides = DEFAULT_OVERRIDES
            .into_iter()
            .filter_map(|(struct_name, field_name, value)| {
                if struct_name == name {
                    Some((field_name, value))
                } else {
                    None
                }
            })
            .collect();

        Self {
            name,
            fields,
            all_items,
            default_overrides,
            file_contents: Vec::new(),
            is_frozen: FROZEN_TYPES.contains(&name),
        }
    }

    fn generate_definition(&mut self) {
        let pyclass_start_str = "#[pyclass(module = \"rlbot_flatbuffers\", subclass, ";
        if self.is_frozen {
            write_fmt!(self, "{pyclass_start_str}frozen, get_all)]");
        } else if self.fields.is_empty() {
            write_fmt!(self, "{pyclass_start_str}frozen)]");
        } else {
            write_fmt!(self, "{pyclass_start_str}get_all)]");
        }

        if self.fields.is_empty() {
            write_str!(self, "#[derive(Default)]");
            write_fmt!(self, "pub struct {} {{}}", self.name);
            write_str!(self, "");
            return;
        }

        write_fmt!(self, "pub struct {} {{", self.name);

        for (field_name, field_info) in self.fields {
            for docstring in &field_info.docstrings.docstrings {
                write_fmt!(self, "    ///{}", docstring.value);
            }

            let mut add_set = !self.is_frozen;
            let variable_type = match &field_info.type_ {
                SimpleType::Bool => Cow::Borrowed("bool"),
                SimpleType::Float(_) => {
                    add_set = false;
                    Cow::Borrowed("Py<PyFloat>")
                }
                SimpleType::Integer(int_type) => Cow::Borrowed(get_int_name(int_type)),
                SimpleType::Struct(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!("Py<super::{name}>"))
                }
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };

            if add_set {
                write_str!(self, "    #[pyo3(set)]");
            }

            write_fmt!(self, "    pub {field_name}: {variable_type},");
        }

        write_str!(self, "}\n");
        write_fmt!(self, "impl crate::PyDefault for {} {{", self.name);
        write_str!(self, "    fn py_default(py: Python) -> Py<Self> {");
        write_str!(self, "        Py::new(py, Self {");

        for (field_name, field_type) in self.fields {
            let end = match &field_type.type_ {
                SimpleType::Float(_) => Cow::Borrowed("crate::pyfloat_default(py)"),
                SimpleType::Struct(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!("super::{name}::py_default(py)"))
                }
                _ => Cow::Borrowed("Default::default()"),
            };

            write_fmt!(self, "            {field_name}: {end},");
        }

        write_str!(self, "        }).unwrap()");
        write_str!(self, "    }");
        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_from_flat_impls(&mut self) {
        let impl_type = format!("flat::{}", self.name);

        if self.fields.is_empty() {
            write_fmt!(self, "impl From<{impl_type}> for {} {{", self.name);
            write_fmt!(self, "    fn from(_: {impl_type}) -> Self {{");
            write_fmt!(self, "        {} {{}}", self.name);
            write_str!(self, "    }");
            write_str!(self, "}");
            write_str!(self, "");
            return;
        }

        write_fmt!(self, "impl FromGil<{impl_type}> for {} {{", self.name);

        write_str!(self, "    #[allow(unused_variables)]");
        write_fmt!(
            self,
            "    fn from_gil(py: Python, flat_t: {impl_type}) -> Self {{"
        );
        write_fmt!(self, "        {} {{", self.name);

        for (field_name, field_info) in self.fields {
            match &field_info.type_ {
                SimpleType::Float(_) => {
                    write_fmt!(
                        self,
                        "            {field_name}: crate::float_to_py(py, flat_t.{field_name}),"
                    )
                }
                SimpleType::Bool | SimpleType::Integer(_) => {
                    write_fmt!(self, "            {field_name}: flat_t.{field_name},")
                }
                SimpleType::Struct(_) => {
                    write_fmt!(
                        self,
                        "            {field_name}: crate::into_py_from(py, flat_t.{field_name}),",
                    );
                }
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_to_flat_impls(&mut self) {
        let impl_type = format!("flat::{}", self.name);

        if self.fields.is_empty() {
            write_fmt!(self, "impl From<&{}> for {impl_type} {{", self.name);
            write_fmt!(self, "    fn from(_: &{}) -> Self {{", self.name);
            write_str!(self, "        Self {}");
            write_str!(self, "    }");
            write_str!(self, "}");
            write_str!(self, "");
            return;
        }

        write_fmt!(self, "impl FromGil<&{}> for {impl_type} {{", self.name);
        write_str!(self, "    #[allow(unused_variables)]");
        write_fmt!(
            self,
            "    fn from_gil(py: Python, py_type: &{}) -> Self {{",
            self.name
        );
        write_str!(self, "        Self {");

        for (field_name, field_info) in self.fields {
            match &field_info.type_ {
                SimpleType::Float(_) => {
                    write_fmt!(
                        self,
                        "            {field_name}: crate::float_from_py(py, &py_type.{field_name}),"
                    );
                }
                SimpleType::Bool | SimpleType::Integer(_) => {
                    write_fmt!(self, "            {field_name}: py_type.{field_name},");
                }
                SimpleType::Struct(_) => {
                    write_fmt!(
                        self,
                        "            {field_name}: crate::from_py_into(py, &py_type.{field_name}),",
                    );
                }
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_new_method(&mut self) {
        write_str!(self, "    #[new]");

        if self.fields.is_empty() {
            write_str!(self, "    pub fn new() -> Self {");
            write_str!(self, "        Self {}");
            write_str!(self, "    }");
            return;
        }

        let mut signature_parts = Vec::new();
        let mut needs_python = false;

        for (field_name, field_info) in self.fields {
            if let Some((_, default_value)) = self
                .default_overrides
                .iter()
                .find(|(override_field_name, _)| override_field_name == field_name)
            {
                signature_parts.push(format!("{field_name}={default_value}"));
                continue;
            }

            signature_parts.push(match &field_info.type_ {
                SimpleType::Bool => format!("{field_name}=false"),
                SimpleType::Integer(_) => format!("{field_name}=0"),
                SimpleType::Float(_) => {
                    needs_python = true;
                    format!("{field_name}=0.0")
                }
                _ => {
                    needs_python = true;
                    format!("{field_name}=None")
                }
            });
        }

        let max_num_types = if needs_python { 6 } else { 7 };
        if self.fields.len() > max_num_types {
            write_str!(self, "    #[allow(clippy::too_many_arguments)]");
        }

        write_fmt!(
            self,
            "    #[pyo3(signature = ({}))]",
            signature_parts.join(", ")
        );
        write_str!(self, "    pub fn new(");

        if needs_python {
            write_str!(self, "        py: Python,");
        }

        for (field_name, field_info) in self.fields {
            let variable_type = match &field_info.type_ {
                SimpleType::Bool => Cow::Borrowed("bool"),
                SimpleType::Integer(int_type) => Cow::Borrowed(get_int_name(int_type)),
                SimpleType::Float(_) => Cow::Borrowed("f64"),
                SimpleType::Struct(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!("Option<Py<super::{name}>>"))
                }
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };

            write_fmt!(self, "        {field_name}: {variable_type},");
        }

        write_str!(self, "    ) -> Self {");
        write_str!(self, "        Self {");

        for (field_name, field_info) in self.fields {
            match &field_info.type_ {
                SimpleType::Struct(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    write_fmt!(
                        self,
                        "            {field_name}: {field_name}.unwrap_or_else(|| super::{name}::py_default(py)),"
                    );
                }
                SimpleType::Float(_) => {
                    write_fmt!(
                        self,
                        "            {field_name}: PyFloat::new(py, {field_name}).unbind(),"
                    )
                }
                _ => write_fmt!(self, "            {field_name},"),
            }
        }

        write_str!(self, "        }");
        write_str!(self, "    }");

        if self.is_frozen {
            return;
        }

        for (field_name, field_info) in self.fields {
            match &field_info.type_ {
                SimpleType::Float(_) => {
                    write_str!(self, "\n    #[setter]");
                    write_fmt!(
                        self,
                        "    pub fn {field_name}(&mut self, py: Python, value: f64) {{",
                    );
                    write_fmt!(
                        self,
                        "        self.{field_name} = PyFloat::new(py, value).unbind();"
                    );
                    write_str!(self, "    }");
                }
                _ => continue,
            }
        }
    }

    fn generate_str_method(&mut self) {
        write_str!(self, "    pub fn __str__(&self, py: Python) -> String {");
        write_str!(self, "        self.__repr__(py)");
        write_str!(self, "    }");
    }

    fn generate_repr_method(&mut self) {
        if self.fields.is_empty() {
            write_str!(self, "    pub fn __repr__(&self, _py: Python) -> String {");
            write_fmt!(self, "        String::from(\"{}()\")", self.name);
            write_str!(self, "    }");
            return;
        }

        write_str!(self, "    #[allow(unused_variables)]");
        write_str!(self, "    pub fn __repr__(&self, py: Python) -> String {");
        write_str!(self, "        format!(");

        let repr_signature = self
            .fields
            .iter()
            .map(|(field_name, _)| format!("{field_name}={{}}"))
            .collect::<Vec<_>>()
            .join(", ");
        write_fmt!(self, "            \"{}({repr_signature})\",", self.name);

        for (field_name, field_info) in self.fields {
            match &field_info.type_ {
                SimpleType::Struct(_) => {
                    write_fmt!(
                        self,
                        "            self.{field_name}.borrow(py).__repr__(py),"
                    );
                }
                SimpleType::Bool => {
                    write_fmt!(self, "            crate::bool_to_str(self.{field_name}),");
                }
                SimpleType::Integer(_) | SimpleType::Float(_) => {
                    write_fmt!(self, "            self.{field_name},");
                }
                _ => write_fmt!(self, "            self.{field_name}.__repr__(),"),
            }
        }

        write_str!(self, "        )");
        write_str!(self, "    }");
    }

    fn generate_long_args(&mut self) {
        write_str!(self, "    #[classattr]");
        write_str!(
            self,
            "    fn __match_args__(py: Python) -> Bound<pyo3::types::PyTuple> {"
        );
        write_str!(self, "        pyo3::types::PyTuple::new(py, [");

        for field_name in self.fields.keys() {
            write_fmt!(self, "            \"{field_name}\",");
        }

        write_str!(self, "        ]).unwrap()");
        write_str!(self, "    }");
    }

    fn generate_args(&mut self) {
        if self.fields.is_empty() {
            return;
        }

        if self.fields.len() > 12 {
            self.generate_long_args();
            return;
        }

        let sig_parts: Vec<_> = repeat_n("&'static str", self.fields.len()).collect();
        let sig = sig_parts.join(", ");

        write_str!(self, "    #[classattr]");
        write_fmt!(self, "    fn __match_args__() -> ({sig},) {{");
        write_str!(self, "        (");

        for field_name in self.fields.keys() {
            write_fmt!(self, "            \"{field_name}\",");
        }

        write_str!(self, "        )");
        write_str!(self, "    }");
    }

    fn generate_py_methods(&mut self) {
        write_str!(self, "#[pymethods]");
        write_fmt!(self, "impl {} {{", self.name);

        self.generate_new_method();
        write_str!(self, "");

        self.generate_str_method();
        write_str!(self, "");

        self.generate_repr_method();
        write_str!(self, "");

        self.generate_args();

        write_str!(self, "}");
        write_str!(self, "");
    }

    pub fn generate_binds(mut self) -> Vec<Cow<'static, str>> {
        self.file_contents
            .push(Cow::Borrowed(if self.fields.is_empty() {
                "use crate::{FromGil, flat};"
            } else {
                "use crate::{FromGil, PyDefault, flat};"
            }));

        write_str!(self, "use pyo3::{prelude::*, types::*};");
        write_str!(self, "");

        self.generate_definition();
        self.generate_from_flat_impls();
        self.generate_to_flat_impls();
        self.generate_py_methods();

        self.file_contents
    }
}
