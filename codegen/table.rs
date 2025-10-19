use crate::{FROZEN_TYPES, get_int_name};
use indexmap::IndexMap;
use planus_types::{
    ast::IntegerType,
    intermediate::{AbsolutePath, AssignMode, Declaration, SimpleType, TableField, TypeKind},
};
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

pub struct TableBindGenerator<'a> {
    name: &'a str,
    fields: &'a IndexMap<String, TableField>,
    all_items: &'a IndexMap<AbsolutePath, Declaration>,
    file_contents: Vec<Cow<'static, str>>,
    is_frozen: bool,
}

impl<'a> TableBindGenerator<'a> {
    pub fn new(
        name: &'a str,
        fields: &'a IndexMap<String, TableField>,
        all_items: &'a IndexMap<AbsolutePath, Declaration>,
    ) -> Self {
        Self {
            name,
            fields,
            all_items,
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
            let mut add_set = !self.is_frozen;
            let variable_type = match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Bool => Cow::Borrowed("bool"),
                    SimpleType::Float(_) => {
                        add_set = false;
                        Cow::Borrowed("Py<PyFloat>")
                    }
                    SimpleType::Integer(int_type) => Cow::Borrowed(get_int_name(int_type)),
                    SimpleType::Struct(idx) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        match path.0.last().unwrap().as_str() {
                            "Float" => {
                                add_set = false;
                                Cow::Borrowed("Py<PyFloat>")
                            }
                            name => Cow::Owned(format!("Py<super::{name}>")),
                        }
                    }
                    SimpleType::Enum(idx) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        let name = path.0.last().unwrap();
                        Cow::Owned(format!("super::{name}"))
                    }
                },
                TypeKind::String => Cow::Borrowed("Py<PyString>"),
                TypeKind::Table(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!("Py<super::{name}>"))
                }
                TypeKind::Vector(inner_type) => match &inner_type.kind {
                    TypeKind::SimpleType(SimpleType::Integer(IntegerType::U8)) => {
                        Cow::Borrowed("Py<PyBytes>")
                    }
                    _ => Cow::Borrowed("Py<PyList>"),
                },
                TypeKind::Union(_) => Cow::Borrowed("Py<PyAny>"),
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };

            if add_set {
                write_str!(self, "    #[pyo3(set)]");
            }

            match field_info.assign_mode {
                AssignMode::Optional => {
                    write_fmt!(self, "    pub {field_name}: Option<{variable_type}>,")
                }
                _ => write_fmt!(self, "    pub {field_name}: {variable_type},"),
            }
        }

        write_str!(self, "}\n");
        write_fmt!(self, "impl crate::PyDefault for {} {{", self.name);
        write_str!(self, "    fn py_default(py: Python) -> Py<Self> {");
        write_str!(self, "        Py::new(py, Self {");

        for (field_name, field_info) in self.fields {
            if matches!(field_info.assign_mode, AssignMode::Optional) {
                write_fmt!(self, "            {field_name}: None,");
                continue;
            }

            let end = match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Float(_) => Cow::Borrowed("crate::pyfloat_default(py)"),
                    SimpleType::Struct(idx) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        let name = path.0.last().unwrap();
                        Cow::Owned(format!("super::{name}::py_default(py)"))
                    }
                    _ => Cow::Borrowed("Default::default()"),
                },
                TypeKind::String => Cow::Borrowed("crate::pydefault_string(py)"),
                TypeKind::Table(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!("super::{name}::py_default(py)"))
                }
                TypeKind::Vector(inner_type) => match &inner_type.kind {
                    TypeKind::SimpleType(SimpleType::Integer(IntegerType::U8)) => {
                        Cow::Borrowed("PyBytes::new(py, &[]).unbind()")
                    }
                    _ => Cow::Borrowed("PyList::empty(py).unbind()"),
                },
                TypeKind::Union(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!("super::{name}::py_default(py)"))
                }
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
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
            let end = match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Float(_) => {
                        format!("crate::float_to_py(py, flat_t.{field_name})")
                    }
                    SimpleType::Bool | SimpleType::Integer(_) => {
                        format!("flat_t.{field_name}")
                    }
                    SimpleType::Struct(idx) => match field_info.assign_mode {
                        AssignMode::Optional => {
                            let (path, _) = self.all_items.get_index(idx.0).unwrap();
                            match path.0.last().unwrap().as_str() {
                                "Float" => format!(
                                    "flat_t.{field_name}.map(|x| crate::float_to_py(py, x.val))"
                                ),
                                _ => format!(
                                    "flat_t.{field_name}.map(|x| crate::into_py_from(py, x))"
                                ),
                            }
                        }
                        _ => {
                            format!("crate::into_py_from(py, flat_t.{field_name})")
                        }
                    },
                    SimpleType::Enum(_) => {
                        format!("flat_t.{field_name}.into()")
                    }
                },
                TypeKind::String => match field_info.assign_mode {
                    AssignMode::Optional => {
                        format!("flat_t.{field_name}.map(|s| PyString::new(py, &s).unbind())")
                    }
                    _ => {
                        format!("PyString::new(py, &flat_t.{field_name}).unbind()")
                    }
                },
                TypeKind::Table(_) => match field_info.assign_mode {
                    AssignMode::Optional => {
                        format!("flat_t.{field_name}.map(|x| crate::into_py_from(py, *x))")
                    }
                    _ => {
                        format!("crate::into_py_from(py, *flat_t.{field_name})")
                    }
                },
                TypeKind::Vector(inner_type) => match &inner_type.kind {
                    TypeKind::String => {
                        format!("crate::into_pystringlist_from(py, flat_t.{field_name})")
                    }
                    TypeKind::SimpleType(SimpleType::Integer(IntegerType::U8)) => {
                        format!("PyBytes::new(py, &flat_t.{field_name}).unbind()")
                    }
                    TypeKind::Table(idx) | TypeKind::SimpleType(SimpleType::Struct(idx)) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        let type_name = path.0.last().unwrap();
                        format!(
                            "PyList::new(py, flat_t.{field_name}.into_iter().map(|x| crate::into_py_from::<_, super::{type_name}>(py, x))).unwrap().unbind()"
                        )
                    }
                    _ => todo!("Unknown field type for {field_name} in {}", self.name),
                },
                TypeKind::Union(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let type_name = path.0.last().unwrap();

                    match field_info.assign_mode {
                        AssignMode::Optional => format!(
                            "flat_t.{field_name}.map(|x| IntoGil::<super::{type_name}>::into_gil(x, py).into_any())"
                        ),
                        _ => format!(
                            "IntoGil::<super::{type_name}>::into_gil(flat_t.{field_name}, py).into_any()"
                        ),
                    }
                }
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };

            write_fmt!(self, "            {field_name}: {end},")
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
            let end = match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Float(_) => {
                        format!("crate::float_from_py(py, &py_type.{field_name})")
                    }
                    SimpleType::Bool | SimpleType::Integer(_) => {
                        format!("py_type.{field_name}")
                    }
                    SimpleType::Struct(idx) => match field_info.assign_mode {
                        AssignMode::Optional => {
                            let (path, _) = self.all_items.get_index(idx.0).unwrap();
                            match path.0.last().unwrap().as_str() {
                                "Float" => format!(
                                    "py_type.{field_name}.as_ref().map(|x| flat::Float {{ val: crate::float_from_py(py, x) }})"
                                ),
                                _ => format!(
                                    "py_type.{field_name}.as_ref().map(|x| crate::from_py_into(py, x))"
                                ),
                            }
                        }
                        _ => {
                            format!("crate::from_py_into(py, &py_type.{field_name})",)
                        }
                    },
                    SimpleType::Enum(_) => {
                        format!("py_type.{field_name}.into()")
                    }
                },
                TypeKind::String => match field_info.assign_mode {
                    AssignMode::Optional => {
                        format!(
                            "py_type.{field_name}.as_ref().map(|s| s.to_str(py).unwrap().to_string())"
                        )
                    }
                    _ => {
                        format!("py_type.{field_name}.to_str(py).unwrap().to_string()",)
                    }
                },
                TypeKind::Table(_) => match field_info.assign_mode {
                    AssignMode::Optional => {
                        format!(
                            "py_type.{field_name}.as_ref().map(|x| Box::new(crate::from_py_into(py, x)))"
                        )
                    }
                    _ => {
                        format!("Box::new(crate::from_py_into(py, &py_type.{field_name}))",)
                    }
                },
                TypeKind::Vector(inner_type) => match &inner_type.kind {
                    TypeKind::String => format!(
                        "py_type.{field_name}.bind_borrowed(py).iter().map(|x| crate::from_pystring_into(x)).collect()"
                    ),
                    TypeKind::SimpleType(SimpleType::Integer(IntegerType::U8)) => {
                        format!("py_type.{field_name}.as_bytes(py).to_vec()")
                    }
                    TypeKind::Table(_) | TypeKind::SimpleType(SimpleType::Struct(_)) => {
                        format!(
                            "py_type.{field_name}.bind_borrowed(py).iter().map(|x| crate::from_pyany_into(py, x)).collect()"
                        )
                    }
                    _ => todo!("Unknown field type for {field_name} in {}", self.name),
                },
                TypeKind::Union(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();

                    match field_info.assign_mode {
                        AssignMode::Optional => {
                            format!(
                                "py_type.{field_name}.as_ref().map(|x| super::{name}::extract(x.bind_borrowed(py)).as_ref().unwrap().into_gil(py))"
                            )
                        }
                        _ => {
                            format!(
                                "super::{name}::extract(py_type.{field_name}.bind_borrowed(py)).as_ref().unwrap().into_gil(py)"
                            )
                        }
                    }
                }
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };

            write_fmt!(self, "            {field_name}: {end},",);
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
            if matches!(field_info.assign_mode, AssignMode::Optional) {
                if let TypeKind::SimpleType(SimpleType::Struct(idx)) = &field_info.type_.kind {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    needs_python |= name.as_str() == "Float";
                }

                signature_parts.push(format!("{field_name}=None"));
                continue;
            }

            signature_parts.push(match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Bool => format!("{field_name}=false"),
                    SimpleType::Integer(_) => format!("{field_name}=0"),
                    SimpleType::Enum(_) => format!("{field_name}=Default::default()"),
                    SimpleType::Float(_) => {
                        needs_python = true;
                        format!("{field_name}=0.0")
                    }
                    SimpleType::Struct(_) => {
                        needs_python = true;
                        format!("{field_name}=None")
                    }
                },
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
            let variable_type = match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Bool => Cow::Borrowed("bool"),
                    SimpleType::Integer(int_type) => Cow::Borrowed(get_int_name(int_type)),
                    SimpleType::Float(_) => Cow::Borrowed("f64"),
                    SimpleType::Enum(idx) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        let name = path.0.last().unwrap();
                        Cow::Owned(format!("super::{name}"))
                    }
                    SimpleType::Struct(idx) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        let name = path.0.last().unwrap();
                        match name.as_str() {
                            "Float" => Cow::Borrowed("Option<f64>"),
                            _ => Cow::Owned(format!("Option<Py<super::{name}>>")),
                        }
                    }
                },
                TypeKind::Table(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!("Option<Py<super::{name}>>"))
                }
                TypeKind::String => Cow::Borrowed("Option<Py<PyString>>"),
                TypeKind::Vector(inner_type) => match inner_type.kind {
                    TypeKind::SimpleType(SimpleType::Integer(IntegerType::U8)) => {
                        Cow::Borrowed("Option<Py<PyBytes>>")
                    }
                    _ => Cow::Borrowed("Option<Py<PyList>>"),
                },
                TypeKind::Union(_) => Cow::Borrowed("Option<Py<PyAny>>"),
                _ => todo!("Unknown field type for {field_name} in {}", self.name),
            };

            write_fmt!(self, "        {field_name}: {variable_type},");
        }

        write_str!(self, "    ) -> Self {");
        write_str!(self, "        Self {");

        for (field_name, field_info) in self.fields {
            if matches!(field_info.assign_mode, AssignMode::Optional) {
                match &field_info.type_.kind {
                    TypeKind::SimpleType(SimpleType::Struct(idx)) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        let name = path.0.last().unwrap();
                        match name.as_str() {
                            "Float" => {
                                write_fmt!(
                                    self,
                                    "            {field_name}: {field_name}.map(|x| PyFloat::new(py, x).unbind()),"
                                );
                            }
                            _ => {
                                write_fmt!(self, "            {field_name},");
                            }
                        }
                    }
                    _ => {
                        write_fmt!(self, "            {field_name},");
                    }
                }
                continue;
            }

            let end = match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Struct(idx) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        let name = path.0.last().unwrap();
                        Cow::Owned(format!(
                            ": {field_name}.unwrap_or_else(|| super::{name}::py_default(py))"
                        ))
                    }
                    SimpleType::Float(_) => {
                        Cow::Owned(format!(": PyFloat::new(py, {field_name}).unbind()"))
                    }
                    _ => Cow::Borrowed(""),
                },
                TypeKind::Table(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!(
                        ": {field_name}.unwrap_or_else(|| super::{name}::py_default(py))"
                    ))
                }
                TypeKind::String => Cow::Owned(format!(
                    ": {field_name}.unwrap_or_else(|| crate::pydefault_string(py))"
                )),
                TypeKind::Vector(inner_type) => match inner_type.kind {
                    TypeKind::SimpleType(SimpleType::Integer(IntegerType::U8)) => Cow::Owned(
                        format!(": {field_name}.unwrap_or_else(|| PyBytes::new(py, &[]).unbind())"),
                    ),
                    _ => Cow::Owned(format!(
                        ": {field_name}.unwrap_or_else(|| PyList::empty(py).unbind())"
                    )),
                },
                TypeKind::Union(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();
                    Cow::Owned(format!(
                        ": {field_name}.unwrap_or_else(|| super::{name}::py_default(py))"
                    ))
                }
                _ => Cow::Borrowed(""),
            };

            write_fmt!(self, "            {field_name}{end},");
        }

        write_str!(self, "        }");
        write_str!(self, "    }");

        if self.is_frozen {
            return;
        }

        for (field_name, field_info) in self.fields {
            match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Float(_) => {}
                    SimpleType::Struct(idx) => {
                        let (path, _) = self.all_items.get_index(idx.0).unwrap();
                        if path.0.last().unwrap().as_str() != "Float" {
                            continue;
                        }
                    }
                    _ => continue,
                },
                _ => continue,
            };

            write_str!(self, "\n    #[setter]");

            let is_optional = matches!(field_info.assign_mode, AssignMode::Optional);
            let value = if is_optional { "Option<f64>" } else { "f64" };

            write_fmt!(
                self,
                "    pub fn {field_name}(&mut self, py: Python, value: {value}) {{",
            );

            let end = if matches!(field_info.assign_mode, AssignMode::Optional) {
                "value.map(|x| PyFloat::new(py, x).unbind())"
            } else {
                "PyFloat::new(py, value).unbind()"
            };

            write_fmt!(self, "        self.{field_name} = {end};");
            write_str!(self, "    }");
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
            .map(|(field_name, field_info)| match &field_info.type_.kind {
                TypeKind::String => {
                    if matches!(field_info.assign_mode, AssignMode::Optional) {
                        format!("{field_name}={{}}")
                    } else {
                        format!("{field_name}={{:?}}")
                    }
                }
                TypeKind::Vector(inner_type) => match inner_type.kind {
                    TypeKind::SimpleType(SimpleType::Integer(IntegerType::U8)) => {
                        format!("{field_name}=bytes([{{}}])")
                    }
                    _ => format!("{field_name}=[{{}}]"),
                },
                _ => format!("{field_name}={{}}"),
            })
            .collect::<Vec<_>>()
            .join(", ");
        write_fmt!(self, "            \"{}({repr_signature})\",", self.name);

        for (field_name, field_info) in self.fields {
            match &field_info.type_.kind {
                TypeKind::SimpleType(simple_type) => match simple_type {
                    SimpleType::Struct(idx) => match field_info.assign_mode {
                        AssignMode::Optional => {
                            write_fmt!(self, "            self.{field_name}");
                            write_str!(self, "                .as_ref()");
                            write_str!(self, "                .map_or_else(crate::none_str, |x| {");

                            let (path, _) = self.all_items.get_index(idx.0).unwrap();
                            match path.0.last().unwrap().as_str() {
                                "Float" => {
                                    write_fmt!(self, "                    x.to_string()");
                                }
                                _ => {
                                    write_fmt!(
                                        self,
                                        "                    x.borrow(py).__repr__(py)"
                                    );
                                }
                            };
                            write_str!(self, "                }),");
                        }
                        _ => {
                            write_fmt!(
                                self,
                                "            self.{field_name}.borrow(py).__repr__(py),"
                            );
                        }
                    },
                    SimpleType::Bool => {
                        write_fmt!(self, "            crate::bool_to_str(self.{field_name}),");
                    }
                    SimpleType::Integer(_) | SimpleType::Float(_) => {
                        write_fmt!(self, "            self.{field_name},");
                    }
                    SimpleType::Enum(_) => {
                        write_fmt!(self, "            self.{field_name}.__repr__(),")
                    }
                },
                TypeKind::String => {
                    if matches!(field_info.assign_mode, AssignMode::Optional) {
                        write_fmt!(self, "            self.{field_name}");
                        write_str!(self, "                .as_ref()");
                        write_str!(self, "                .map_or_else(crate::none_str, |i| {");
                        write_str!(
                            self,
                            "                    crate::format_string(i.to_str(py).unwrap().to_string())"
                        );
                        write_str!(self, "                }),");
                    } else {
                        write_fmt!(
                            self,
                            "            self.{field_name}.bind(py).to_cow().unwrap(),"
                        );
                    }
                }
                TypeKind::Table(_) => match field_info.assign_mode {
                    AssignMode::Optional => {
                        write_fmt!(self, "            self.{field_name}");
                        write_str!(self, "                .as_ref()");
                        write_str!(self, "                .map_or_else(crate::none_str, |x| {");
                        write_str!(self, "                    x.borrow(py).__repr__(py)");
                        write_str!(self, "                }),");
                    }
                    _ => {
                        write_fmt!(
                            self,
                            "            self.{field_name}.borrow(py).__repr__(py),"
                        );
                    }
                },
                TypeKind::Union(idx) => {
                    let (path, _) = self.all_items.get_index(idx.0).unwrap();
                    let name = path.0.last().unwrap();

                    match field_info.assign_mode {
                        AssignMode::Optional => {
                            write_fmt!(
                                self,
                                "            self.{field_name}.as_ref().map_or_else(crate::none_str, |i| {{"
                            );
                            write_fmt!(
                                self,
                                "                super::{name}::extract(i.bind_borrowed(py))"
                            );
                            write_str!(self, "                .unwrap().__repr__(py)");
                            write_str!(self, "            }),");
                        }
                        _ => {
                            write_fmt!(
                                self,
                                "            super::{name}::extract(self.{field_name}.bind_borrowed(py))"
                            );
                            write_str!(self, "                .unwrap().__repr__(py),");
                        }
                    }
                }
                TypeKind::Vector(inner_type) => {
                    write_fmt!(self, "            self.{field_name}");

                    match inner_type.kind {
                        TypeKind::SimpleType(simple_type) => match simple_type {
                            SimpleType::Integer(IntegerType::U8) => {
                                write_str!(self, "                .as_bytes(py)");
                                write_str!(self, "                .iter()");
                                write_str!(self, "                .map(ToString::to_string)");
                            }
                            SimpleType::Struct(idx) => {
                                let (path, _) = self.all_items.get_index(idx.0).unwrap();
                                let name = path.0.last().unwrap();
                                write_str!(self, "                .bind_borrowed(py)");
                                write_str!(self, "                .iter()");
                                write_fmt!(
                                    self,
                                    "                .map(|x| x.cast_into::<super::{name}>().unwrap().borrow().__repr__(py))"
                                );
                            }
                            _ => {
                                write_str!(self, "                .iter()");
                                write_str!(self, "                .map(ToString::to_string)");
                            }
                        },
                        TypeKind::String => {
                            write_str!(self, "                .bind_borrowed(py)");
                            write_str!(self, "                .iter()");
                            write_str!(
                                self,
                                "                .map(|s| crate::format_string(crate::from_pystring_into(s)))"
                            );
                        }
                        TypeKind::Table(idx) => {
                            let (path, _) = self.all_items.get_index(idx.0).unwrap();
                            let name = path.0.last().unwrap();
                            write_str!(self, "                .bind_borrowed(py)");
                            write_str!(self, "                .iter()");
                            write_fmt!(
                                self,
                                "                .map(|x| x.cast_into::<super::{name}>().unwrap().borrow().__repr__(py))"
                            );
                        }
                        _ => continue,
                    }

                    write_str!(self, "                .collect::<Vec<String>>()");
                    write_str!(self, "                .join(\", \"),");
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
        write_str!(self, "    }\n");
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
        write_str!(self, "    }\n");
    }

    fn generate_pack_method(&mut self) {
        write_str!(
            self,
            "    fn pack<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {"
        );
        write_str!(
            self,
            "        let mut builder = Builder::with_capacity(u16::MAX as usize);\n"
        );
        write_fmt!(
            self,
            "        let flat_t = flat::{}::from_gil(py, self);",
            self.name
        );
        write_str!(
            self,
            "        PyBytes::new(py, builder.finish(flat_t, None))"
        );
        write_str!(self, "    }");
    }

    fn generate_unpack_method(&mut self) {
        write_str!(self, "    #[staticmethod]");
        write_str!(
            self,
            "    fn unpack(py: Python, data: &[u8]) -> PyResult<Py<Self>> {"
        );
        write_fmt!(
            self,
            "        let flat_t_ref = flat::{}Ref::read_as_root(data).map_err(flat_err_to_py)?;",
            self.name
        );
        write_fmt!(
            self,
            "        let flat_t = flat::{}::try_from(flat_t_ref).map_err(flat_err_to_py)?;\n",
            self.name
        );
        write_str!(self, "        Ok(crate::into_py_from(py, flat_t))");
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

        self.generate_pack_method();
        write_str!(self, "");

        self.generate_unpack_method();
        write_str!(self, "}");
        write_str!(self, "");
    }

    pub fn generate_binds(mut self) -> Vec<Cow<'static, str>> {
        self.file_contents
            .push(Cow::Borrowed(if self.fields.is_empty() {
                "use crate::{FromGil, flat_err_to_py, flat};"
            } else {
                "use crate::{FromGil, IntoGil, PyDefault, flat, flat_err_to_py};"
            }));

        write_str!(self, "use planus::{Builder, ReadAsRoot};");
        write_str!(self, "use pyo3::{prelude::*, types::*};");
        write_str!(self, "");

        self.generate_definition();
        self.generate_from_flat_impls();
        self.generate_to_flat_impls();
        self.generate_py_methods();

        self.file_contents
    }
}
