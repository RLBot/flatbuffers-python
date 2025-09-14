use indexmap::IndexMap;
use planus_types::intermediate::{EnumVariant, IntegerLiteral};
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

/// Examples of what this function does:
/// FriendlyFire => FriendlyFire
/// ContactFF => ContactFf
/// ContactSilent => ContactSilent
/// ContactFFSilent => ContactFfSilent
///
/// If a string doesn't need to be updated, the original is returned
pub fn normalize_caps(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let mut i = 0;

    // check if changes need to be made
    // if a change is be needed,
    // `i` will be the location of where we need to start
    while i < bytes.len() - 1 {
        if bytes[i].is_ascii_uppercase() && bytes[i + 1].is_ascii_uppercase() {
            if i + 2 == bytes.len() {
                break;
            }

            if bytes[i + 2].is_ascii_uppercase() {
                break;
            }
        }

        i += 1;
    }

    if i == bytes.len() - 1 {
        // no changes need to be made - return the original
        return Cow::Borrowed(input);
    }

    // changes must be made, `i` stores the location of the first change
    let mut result = String::with_capacity(bytes.len());
    result.push_str(&input[..i]);

    let mut chars = input.chars().skip(i);
    while let Some(mut char) = chars.next() {
        let mut num_upper = 0;

        loop {
            let Some(next_char) = chars.next() else {
                result.push(char.to_ascii_lowercase());
                break;
            };

            if next_char.is_ascii_uppercase() {
                num_upper += 1;
                result.push(if num_upper == 1 {
                    char
                } else {
                    char.to_ascii_lowercase()
                });
                char = next_char;
            } else {
                result.push(char);
                result.push(next_char);
                break;
            }
        }
    }

    Cow::Owned(result)
}

pub struct EnumBindGenerator<'a> {
    name: &'a str,
    variants: &'a IndexMap<IntegerLiteral, EnumVariant>,
    file_contents: Vec<Cow<'static, str>>,
}

impl<'a> EnumBindGenerator<'a> {
    pub fn new(name: &'a str, variants: &'a IndexMap<IntegerLiteral, EnumVariant>) -> Self {
        Self {
            name,
            variants,
            file_contents: Vec::new(),
        }
    }

    fn generate_definition(&mut self) {
        write_str!(
            self,
            "#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]"
        );
        write_str!(
            self,
            "#[pyclass(module = \"rlbot_flatbuffers\", frozen, hash, eq, eq_int)]"
        );
        write_fmt!(self, "pub enum {} {{", self.name);
        write_str!(self, "    #[default]");

        for (var_num, var_info) in self.variants {
            write_fmt!(self, "    {} = {var_num},", normalize_caps(&var_info.name));
        }

        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_from_flat_impls(&mut self) {
        write_fmt!(self, "impl From<flat::{}> for {} {{", self.name, self.name);
        write_fmt!(self, "    fn from(flat_t: flat::{}) -> Self {{", self.name);
        write_str!(self, "        match flat_t {");

        for var_info in self.variants.values() {
            let var_name = normalize_caps(&var_info.name);
            write_fmt!(
                self,
                "            flat::{}::{var_name} => Self::{var_name},",
                self.name
            );
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
        write_str!(self, "}");
        write_str!(self, "");
    }

    fn generate_to_flat_impls(&mut self) {
        write_fmt!(self, "impl From<{}> for flat::{} {{", self.name, self.name);
        write_fmt!(self, "    fn from(py_type: {}) -> Self {{", self.name);
        write_str!(self, "        match py_type {");

        for var_info in self.variants.values() {
            let var_name = normalize_caps(&var_info.name);
            write_fmt!(
                self,
                "            {}::{var_name} => Self::{var_name},",
                self.name,
            );
        }

        write_str!(self, "        }");
        write_str!(self, "    }");
        write_str!(self, "}");

        write_str!(self, "");
    }

    fn generate_new_method(&mut self) {
        write_str!(self, "    #[new]");
        assert!(u8::try_from(self.variants.len()).is_ok());

        write_str!(self, "    #[pyo3(signature = (value=Default::default()))]");
        write_str!(self, "    pub fn new(value: u8) -> PyResult<Self> {");
        write_str!(self, "        match value {");

        for (var_num, var_info) in self.variants {
            write_fmt!(
                self,
                "            {} => Ok(Self::{}),",
                var_num.to_u64(),
                normalize_caps(&var_info.name)
            );
        }

        write_str!(
            self,
            "            v => Err(PyValueError::new_err(format!(\"Unknown value of {v}\"))),"
        );

        write_str!(self, "        }");
        write_str!(self, "    }");
    }

    fn generate_str_method(&mut self) {
        write_str!(self, "    pub fn __str__(&self) -> String {");
        write_str!(self, "        self.__repr__()");
        write_str!(self, "    }");
    }

    fn generate_repr_method(&mut self) {
        write_str!(self, "    pub fn __repr__(&self) -> String {");
        write_fmt!(self, "        format!(\"{}.{{self:?}}\")", self.name);
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
        write_str!(self, "}");
        write_str!(self, "");
    }

    pub fn generate_binds(mut self) -> Vec<Cow<'static, str>> {
        write_str!(self, "use crate::flat;");
        write_str!(
            self,
            "use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};"
        );
        write_str!(self, "");

        self.generate_definition();
        self.generate_from_flat_impls();
        self.generate_to_flat_impls();
        self.generate_py_methods();

        self.file_contents
    }
}
