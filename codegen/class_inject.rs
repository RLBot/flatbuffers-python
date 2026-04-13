use std::{fs, io};

pub fn classes_to_lib_rs(mut class_names: Vec<&str>) -> io::Result<()> {
    class_names.sort_unstable();
    let file_contents = format!(
        "    classes: [\n        {}\n    ],",
        class_names.join(",\n        ")
    );

    let mut lib_rs = fs::read_to_string("src/lib.rs")?;

    #[cfg(windows)]
    {
        lib_rs = lib_rs.replace("\r\n", "\n");
    }

    let start = lib_rs.find("    classes: [\n").unwrap();
    let end = lib_rs[start..].find("],").unwrap() + 2;

    lib_rs.replace_range(start..start + end, &file_contents);
    fs::write("src/lib.rs", lib_rs)?;

    Ok(())
}

pub fn add_pyclass_to_planus_enum(file: &mut String, name: &str) {
    let find_str = format!("pub enum {name} {{");
    let file_pos = file.find(&find_str).unwrap();
    let end_pos = file_pos + find_str.len();
    let start_pos = file[..file_pos].rfind("#[derive").unwrap();
    file.replace_range(
        start_pos..end_pos,
        &format!(
            "#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]\n#[repr(u8)]\n#[::pyo3::pyclass(module = \"rlbot_flatbuffers\", from_py_object, frozen, hash, eq, eq_int)]\n{find_str}#[default]"
        )
    );
}
