use crate::{enums::normalize_caps, structs::DEFAULT_OVERRIDES};
use planus_types::{
    ast::IntegerType,
    intermediate::{AssignMode, DeclarationKind, Declarations, SimpleType, TypeKind},
};
use std::{borrow::Cow, fs, io};

macro_rules! write_str {
    ($self:ident, $s:expr) => {
        $self.push(Cow::Borrowed($s))
    };
}

macro_rules! write_fmt {
    ($self:ident, $($arg:tt)*) => {
        $self.push(Cow::Owned(format!($($arg)*)))
    };
}

pub fn generator(type_data: &Declarations) -> io::Result<()> {
    let mut file = vec![
        Cow::Borrowed("from __future__ import annotations"),
        Cow::Borrowed(""),
        Cow::Borrowed("from typing import Sequence"),
        Cow::Borrowed(""),
        Cow::Borrowed("__doc__: str"),
        Cow::Borrowed("__version__: str"),
        Cow::Borrowed(""),
        Cow::Borrowed("class InvalidFlatbuffer(ValueError): ..."),
        Cow::Borrowed(""),
    ];

    let mut sorted_types: Vec<_> = type_data.iter_declarations().collect();
    sorted_types.sort_by_cached_key(|(idx, full_type_name, item)| match &item.kind {
        DeclarationKind::Enum(_) => (0, 0, 0, 0, full_type_name.0.last().unwrap().as_str()),
        DeclarationKind::Struct(item) => (
            1,
            item.fields
                .iter()
                .any(|(_, item)| matches!(item.type_, SimpleType::Struct(_))) as usize,
            0,
            0,
            full_type_name.0.last().unwrap().as_str(),
        ),
        DeclarationKind::Table(_) | DeclarationKind::Union(_) => (
            2,
            0,
            usize::MAX - type_data.parents[idx.0].len(),
            type_data.children[idx.0].len(),
            full_type_name.0.last().unwrap().as_str(),
        ),
        _ => unreachable!(),
    });

    for (_, full_type_name, item) in sorted_types {
        if matches!(item.kind, DeclarationKind::Union(_)) {
            continue;
        }

        let type_name = full_type_name.0.last().unwrap();

        write_fmt!(file, "class {type_name}:");

        if !item.docstrings.docstrings.is_empty() {
            write_str!(file, "    \"\"\"");

            for docstring in &item.docstrings.docstrings {
                write_fmt!(file, "    {}", docstring.value.trim());
            }

            write_str!(file, "    \"\"\"\n");
        }

        match &item.kind {
            DeclarationKind::Enum(info) => {
                for (var_val, var_info) in &info.variants {
                    let field_name = normalize_caps(&var_info.name);
                    write_fmt!(file, "    {field_name}: {type_name}");

                    write_str!(file, "    \"\"\"");
                    write_fmt!(file, "    `assert int({type_name}.{field_name}) == {var_val}`");

                    if !var_info.docstrings.docstrings.is_empty() {
                        write_str!(file, "");

                        for line in &var_info.docstrings.docstrings {
                            write_fmt!(file, "    {}", line.value.trim());
                        }
                    }

                    write_str!(file, "    \"\"\"");
                }

                write_str!(file, "");
                write_fmt!(
                    file,
                    "    def __new__(cls, value: int = 0) -> {type_name}: ..."
                );
                write_str!(file, "    def __init__(self, value: int = 0) -> None:");
                write_str!(file, "        \"\"\"");
                write_str!(
                    file,
                    "        :raises ValueError: If the `value` is not a valid enum value"
                );
                write_str!(file, "        \"\"\"");
                write_str!(file, "    def __int__(self) -> int: ...");
                write_str!(file, "    def __eq__(self, other) -> bool: ...");
                write_str!(file, "    def __hash__(self) -> int: ...");
            }
            DeclarationKind::Struct(info) => {
                for (field_name, field_info) in &info.fields {
                    let python_type = Cow::Borrowed(match &field_info.type_ {
                        SimpleType::Bool => "bool",
                        SimpleType::Float(_) => "float",
                        SimpleType::Integer(_) => "int",
                        SimpleType::Enum(idx) => {
                            let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                            path.0.last().unwrap()
                        }
                        SimpleType::Struct(idx) => {
                            let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                            path.0.last().unwrap()
                        }
                    });

                    write_fmt!(file, "    {field_name}: {python_type}");

                    if !field_info.docstrings.docstrings.is_empty() {
                        write_str!(file, "    \"\"\"");

                        for line in &field_info.docstrings.docstrings {
                            write_fmt!(file, "    {}", line.value.trim());
                        }

                        write_str!(file, "    \"\"\"");
                    }
                }

                if info.fields.is_empty() {
                    write_fmt!(file, "    def __new__(cls) -> {type_name}: ...");
                    write_str!(file, "    def __init__(self) -> None: ...\n");
                    continue;
                }

                write_str!(file, "");
                write_str!(file, "    __match_args__ = (");

                for field_name in info.fields.keys() {
                    write_fmt!(file, "        \"{field_name}\",");
                }
                write_str!(file, "    )");
                write_str!(file, "");

                let inits = [("new", "cls", type_name.as_str()), ("init", "self", "None")];

                let default_overrides: Vec<_> = DEFAULT_OVERRIDES
                    .into_iter()
                    .filter_map(|(struct_name, field_name, value)| {
                        if struct_name == type_name {
                            Some((field_name, value))
                        } else {
                            None
                        }
                    })
                    .collect();

                for (func, first_arg, ret_type) in inits {
                    write_fmt!(file, "    def __{func}__(");
                    write_fmt!(file, "        {first_arg},");

                    for (field_name, field_info) in &info.fields {
                        let python_type = Cow::Borrowed(match &field_info.type_ {
                            SimpleType::Bool => "bool",
                            SimpleType::Float(_) => "float",
                            SimpleType::Integer(_) => "int",
                            SimpleType::Enum(idx) => {
                                let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                                path.0.last().unwrap()
                            }
                            SimpleType::Struct(idx) => {
                                let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                                path.0.last().unwrap()
                            }
                        });

                        if let Some((_, value)) = default_overrides
                            .iter()
                            .find(|(field, _)| field == field_name)
                        {
                            write_fmt!(file, "        {field_name}: {python_type} = {value},");
                            continue;
                        }

                        let default_value = match &field_info.type_ {
                            SimpleType::Bool => Cow::Borrowed("False"),
                            SimpleType::Float(_) => Cow::Borrowed("0.0"),
                            SimpleType::Integer(_) => Cow::Borrowed("0"),
                            SimpleType::Enum(idx) | SimpleType::Struct(idx) => {
                                let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                                let name = path.0.last().unwrap();
                                Cow::Owned(format!("{name}()"))
                            }
                        };

                        write_fmt!(
                            file,
                            "        {field_name}: {python_type} = {default_value},"
                        );
                    }

                    write_fmt!(file, "    ) -> {ret_type}: ...");
                }
            }
            DeclarationKind::Table(info) => {
                for (field_name, field_info) in &info.fields {
                    let mut python_type = match &field_info.type_.kind {
                        TypeKind::SimpleType(simple_type) => Cow::Borrowed(match simple_type {
                            SimpleType::Bool => "bool",
                            SimpleType::Float(_) => "float",
                            SimpleType::Integer(_) => "int",
                            SimpleType::Enum(idx) => {
                                let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                                path.0.last().unwrap()
                            }
                            SimpleType::Struct(idx) => {
                                let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                                match path.0.last().unwrap().as_str() {
                                    "Float" => "float",
                                    name => name,
                                }
                            }
                        }),
                        TypeKind::Union(idx) => {
                            let (_, info) = type_data.declarations.get_index(idx.0).unwrap();
                            let DeclarationKind::Union(union_info) = &info.kind else {
                                unreachable!()
                            };

                            let mut keys: Vec<_> = union_info.variants.keys().cloned().collect();
                            keys.sort_unstable();

                            Cow::Owned(keys.join(" | "))
                        }
                        TypeKind::Table(idx) => {
                            let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                            Cow::Borrowed(path.0.last().unwrap().as_str())
                        }
                        TypeKind::String => Cow::Borrowed("str"),
                        TypeKind::Vector(inner_type) => match inner_type.kind {
                            TypeKind::SimpleType(simple_type) => match simple_type {
                                SimpleType::Bool => Cow::Borrowed("Sequence[bool]"),
                                SimpleType::Float(_) => Cow::Borrowed("Sequence[float]"),
                                SimpleType::Integer(IntegerType::U8) => Cow::Borrowed("bytes"),
                                SimpleType::Integer(_) => Cow::Borrowed("Sequence[int]"),
                                SimpleType::Enum(idx) | SimpleType::Struct(idx) => {
                                    let (path, _) =
                                        type_data.declarations.get_index(idx.0).unwrap();
                                    let name = path.0.last().unwrap().as_str();
                                    Cow::Owned(format!("Sequence[{name}]"))
                                }
                            },
                            TypeKind::String => Cow::Borrowed("Sequence[str]"),
                            TypeKind::Table(idx) => {
                                let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                                let name = path.0.last().unwrap().as_str();
                                Cow::Owned(format!("Sequence[{name}]"))
                            }
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    };

                    if matches!(field_info.assign_mode, AssignMode::Optional) {
                        let mut new_type = python_type.to_string();
                        new_type.push_str(" | None");
                        python_type = Cow::Owned(new_type);
                    }

                    write_fmt!(file, "    {field_name}: {python_type}");

                    if !field_info.docstrings.docstrings.is_empty() {
                        write_str!(file, "    \"\"\"");

                        for line in &field_info.docstrings.docstrings {
                            write_fmt!(file, "    {}", line.value.trim());
                        }

                        write_str!(file, "    \"\"\"");
                    }
                }

                if info.fields.is_empty() {
                    write_fmt!(file, "    def __new__(cls) -> {type_name}: ...");
                    write_str!(file, "    def __init__(self) -> None: ...\n");
                    continue;
                }

                write_str!(file, "");
                write_str!(file, "    __match_args__ = (");

                for field_name in info.fields.keys() {
                    write_fmt!(file, "        \"{field_name}\",");
                }
                write_str!(file, "    )");
                write_str!(file, "");

                let inits = [("new", "cls", type_name.as_str()), ("init", "self", "None")];

                for (func, first_arg, ret_type) in inits {
                    write_fmt!(file, "    def __{func}__(");
                    write_fmt!(file, "        {first_arg},");

                    for (field_name, field_info) in &info.fields {
                        let mut python_type = match &field_info.type_.kind {
                            TypeKind::SimpleType(simple_type) => Cow::Borrowed(match simple_type {
                                SimpleType::Bool => "bool",
                                SimpleType::Float(_) => "float",
                                SimpleType::Integer(_) => "int",
                                SimpleType::Enum(idx) => {
                                    let (path, _) =
                                        type_data.declarations.get_index(idx.0).unwrap();
                                    path.0.last().unwrap()
                                }
                                SimpleType::Struct(idx) => {
                                    let (path, _) =
                                        type_data.declarations.get_index(idx.0).unwrap();
                                    match path.0.last().unwrap().as_str() {
                                        "Float" => "float",
                                        name => name,
                                    }
                                }
                            }),
                            TypeKind::Table(idx) => {
                                let (path, _) = type_data.declarations.get_index(idx.0).unwrap();
                                Cow::Borrowed(path.0.last().unwrap().as_str())
                            }
                            TypeKind::Union(idx) => {
                                let (_, info) = type_data.declarations.get_index(idx.0).unwrap();
                                let DeclarationKind::Union(union_info) = &info.kind else {
                                    unreachable!()
                                };

                                let mut keys: Vec<_> =
                                    union_info.variants.keys().cloned().collect();
                                keys.sort_unstable();

                                Cow::Owned(keys.join(" | "))
                            }
                            TypeKind::String => Cow::Borrowed("str"),
                            TypeKind::Vector(inner_type) => match inner_type.kind {
                                TypeKind::SimpleType(simple_type) => match simple_type {
                                    SimpleType::Bool => Cow::Borrowed("Sequence[bool]"),
                                    SimpleType::Float(_) => Cow::Borrowed("Sequence[float]"),
                                    SimpleType::Integer(IntegerType::U8) => Cow::Borrowed("bytes"),
                                    SimpleType::Integer(_) => Cow::Borrowed("Sequence[int]"),
                                    SimpleType::Enum(idx) | SimpleType::Struct(idx) => {
                                        let (path, _) =
                                            type_data.declarations.get_index(idx.0).unwrap();
                                        let name = path.0.last().unwrap().as_str();
                                        Cow::Owned(format!("Sequence[{name}]"))
                                    }
                                },
                                TypeKind::String => Cow::Borrowed("Sequence[str]"),
                                TypeKind::Table(idx) => {
                                    let (path, _) =
                                        type_data.declarations.get_index(idx.0).unwrap();
                                    let name = path.0.last().unwrap().as_str();
                                    Cow::Owned(format!("Sequence[{name}]"))
                                }
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        };

                        let default_value =
                            if matches!(field_info.assign_mode, AssignMode::Optional) {
                                let mut new_type = python_type.to_string();
                                new_type.push_str(" | None");
                                python_type = Cow::Owned(new_type);

                                Cow::Borrowed("None")
                            } else {
                                match &field_info.type_.kind {
                                    TypeKind::SimpleType(simple_type) => match simple_type {
                                        SimpleType::Bool => Cow::Borrowed("False"),
                                        SimpleType::Float(_) => Cow::Borrowed("0.0"),
                                        SimpleType::Integer(_) => Cow::Borrowed("0"),
                                        SimpleType::Enum(idx) | SimpleType::Struct(idx) => {
                                            let (path, _) =
                                                type_data.declarations.get_index(idx.0).unwrap();
                                            let name = path.0.last().unwrap();
                                            Cow::Owned(format!("{name}()"))
                                        }
                                    },
                                    TypeKind::String => Cow::Borrowed("\"\""),
                                    TypeKind::Vector(inner_type) => match &inner_type.kind {
                                        TypeKind::SimpleType(SimpleType::Integer(
                                            IntegerType::U8,
                                        )) => Cow::Borrowed("bytes()"),
                                        _ => Cow::Borrowed("[]"),
                                    },
                                    TypeKind::Table(idx) => {
                                        let (path, _) =
                                            type_data.declarations.get_index(idx.0).unwrap();
                                        let name = path.0.last().unwrap();
                                        Cow::Owned(format!("{name}()"))
                                    }
                                    TypeKind::Union(idx) => {
                                        let (_, info) =
                                            type_data.declarations.get_index(idx.0).unwrap();
                                        let DeclarationKind::Union(union_info) = &info.kind else {
                                            unreachable!()
                                        };

                                        let mut keys: Vec<_> = union_info.variants.keys().collect();
                                        keys.sort_unstable();

                                        Cow::Owned(format!("{}()", keys[0]))
                                    }
                                    _ => unimplemented!(),
                                }
                            };

                        write_fmt!(
                            file,
                            "        {field_name}: {python_type} = {default_value},"
                        );
                    }

                    write_fmt!(file, "    ) -> {ret_type}: ...");
                }

                write_str!(file, "    def pack(self) -> bytes:");
                write_str!(file, "        \"\"\"");
                write_str!(file, "        Serializes this instance into a byte array");
                write_str!(file, "        \"\"\"\n");

                write_str!(file, "    @staticmethod");
                write_fmt!(file, "    def unpack(data: bytes) -> {type_name}:");
                write_str!(file, "        \"\"\"");
                write_str!(file, "        Deserializes the data into a new instance\n");
                write_str!(
                    file,
                    "        :raises InvalidFlatbuffer: If the `data` is invalid for this type"
                );
                write_str!(file, "        \"\"\"\n");
            }
            _ => unimplemented!(),
        }

        write_str!(file, "    def __str__(self) -> str: ...");
        write_str!(file, "    def __repr__(self) -> str: ...");
        write_str!(file, "");
    }

    fs::write("rlbot_flatbuffers.pyi", file.join("\n"))?;

    Ok(())
}
