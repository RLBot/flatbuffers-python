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
