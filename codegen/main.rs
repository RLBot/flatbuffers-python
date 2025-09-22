use eyre::{Context, ContextCompat};
use planus_types::{ast::IntegerType, intermediate::DeclarationKind};
use std::{
    env::set_current_dir,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

use crate::{
    enums::EnumBindGenerator, structs::StructBindGenerator, table::TableBindGenerator,
    unions::UnionBindGenerator,
};

mod class_inject;
mod enums;
mod pyi;
mod structs;
mod table;
mod unions;

const SCHEMA_FOLDER: &str = "./flatbuffers-schema";
const SCHEMA_FOLDER_BACKUP: &str = "../flatbuffers-schema";
const RLBOT_FBS: &str = "schema/rlbot.fbs";

const OUT_FILE: &str = "./src/planus_flat.rs";
pub const PYTHON_OUT_FOLDER: &str = "./src/python";

pub const FROZEN_TYPES: [&str; 26] = [
    "ControllableInfo",
    "ControllableTeamInfo",
    "PredictionSlice",
    "BallPrediction",
    "GoalInfo",
    "BoostPad",
    "FieldInfo",
    "Physics",
    "GamePacket",
    "PlayerInfo",
    "ScoreInfo",
    "BallInfo",
    "Touch",
    "CollisionShape",
    "BoxShape",
    "SphereShape",
    "CylinderShape",
    "BoostPadState",
    "MatchInfo",
    "TeamInfo",
    "Vector2",
    "CoreMessage",
    "InterfaceMessage",
    "CorePacket",
    "InterfacePacket",
    "PlayerInput",
];

pub fn get_int_name(int_type: &IntegerType) -> &'static str {
    match int_type {
        IntegerType::U8 => "u8",
        IntegerType::U16 => "u16",
        IntegerType::U32 => "u32",
        IntegerType::U64 => "u64",
        IntegerType::I8 => "i8",
        IntegerType::I16 => "i16",
        IntegerType::I32 => "i32",
        IntegerType::I64 => "i64",
    }
}

fn camel_to_snake(input: &str) -> String {
    let mut snake_case = String::new();

    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                snake_case.push('_');
            }
            snake_case.push(ch.to_ascii_lowercase());
        } else {
            snake_case.push(ch);
        }
    }

    snake_case
}

/// Taken from <https://github.com/planus-org/planus/blob/main/crates/planus-codegen/src/rust/mod.rs#L1014>
///
/// This formats a string using `rustfmt` (using Rust 2024 and not 2021)
fn format_string(s: &str) -> eyre::Result<String> {
    let mut child = Command::new("rustfmt");

    child
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child
        .spawn()
        .wrap_err("Unable to spawn rustfmt. Perhaps it is not installed?")?;

    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin
            .write_all(s.as_bytes())
            .wrap_err("Unable to write the file to rustfmt")?;
    }

    let output = child
        .wait_with_output()
        .wrap_err("Unable to get the formatted file back from rustfmt")?;

    if output.status.success() && output.stderr.is_empty() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else if output.stderr.is_empty() {
        eyre::bail!("rustfmt failed with exit code {}", output.status);
    } else {
        eyre::bail!(
            "rustfmt failed with exit code {} and message:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr).into_owned(),
        )
    }
}

fn main() -> eyre::Result<()> {
    set_current_dir(env!("CARGO_MANIFEST_DIR")).unwrap();

    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/color.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/comms.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/corepacket.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/gamedata.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/gamestatemanip.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/interfacepacket.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/matchconfig.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/misc.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/rendering.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/rlbot.fbs");
    println!("cargo:rerun-if-changed=flatbuffers-schema/schema/vector.fbs");

    let mut schema_folder = Path::new(SCHEMA_FOLDER);
    if !schema_folder.exists() {
        schema_folder = Path::new(SCHEMA_FOLDER_BACKUP);
        assert!(
            schema_folder.exists(),
            "Could not find flatbuffers schema folder"
        );
    }

    let rlbot_fbs_path = schema_folder.join(RLBOT_FBS);
    let declarations = planus_translation::translate_files(&[rlbot_fbs_path.as_path()])
        .context("planus translation failed")?;

    let python_folder = Path::new(PYTHON_OUT_FOLDER);
    if !python_folder.exists() {
        fs::create_dir(python_folder)?;
    }

    let mut python_mod: Vec<String> = Vec::with_capacity(declarations.declarations.len());
    let mut class_names: Vec<&str> = Vec::with_capacity(declarations.declarations.len());
    let mut python_files = Vec::with_capacity(declarations.declarations.len() + 1);
    python_files.push(String::from("mod.rs"));

    let items: Vec<_> = declarations.declarations.keys().collect();

    // generate custom code
    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();

        let num_codegen_threads = thread::available_parallelism().unwrap().get();
        let num_items_per_thread = items.len().div_ceil(num_codegen_threads).max(8);
        let num_threads = items.len().div_ceil(num_items_per_thread);

        for i in 0..num_threads {
            let start_num = num_items_per_thread * i;
            let end_num = (start_num + num_items_per_thread).min(items.len());

            let declarations = &declarations.declarations;
            let items = &items[start_num..end_num];
            let tx = tx.clone();

            s.spawn(move || {
                for &path in items {
                    let item = &declarations[path];
                    let item_name = path.0.last().unwrap().as_str();
                    if item_name == "Float" {
                        // Special case for Float (we always inline Float into Py<PyFloat>)
                        continue;
                    }

                    let mut file_name = camel_to_snake(item_name);

                    let (class_name, file_contents) = match &item.kind {
                        DeclarationKind::Table(info) => {
                            let bind_gen =
                                TableBindGenerator::new(item_name, &info.fields, declarations);
                            (Some(item_name), bind_gen.generate_binds())
                        }
                        DeclarationKind::Struct(info) => {
                            let bind_gen =
                                StructBindGenerator::new(item_name, &info.fields, declarations);
                            (Some(item_name), bind_gen.generate_binds())
                        }
                        DeclarationKind::Enum(info) => {
                            let bind_gen = EnumBindGenerator::new(item_name, &info.variants);
                            (Some(item_name), bind_gen.generate_binds())
                        }
                        DeclarationKind::Union(info) => {
                            let bind_gen = UnionBindGenerator::new(item_name, &info.variants);
                            (None, bind_gen.generate_binds())
                        }
                        DeclarationKind::RpcService(_) => unimplemented!(),
                    };

                    let mod_lines: String =
                        ["mod ", &file_name, ";\n", "pub use ", &file_name, "::*;\n"]
                            .into_iter()
                            .collect();
                    file_name.push_str(".rs");

                    fs::write(
                        python_folder.join(&file_name),
                        format_string(&file_contents.join("\n")).unwrap(),
                    )
                    .unwrap();

                    tx.send((class_name, mod_lines, file_name)).unwrap();
                }
            });
        }

        drop(tx);

        for (class_name, mod_lines, file_name) in rx.iter() {
            if let Some(class_name) = class_name {
                class_names.push(class_name);
            }

            python_mod.push(mod_lines);
            python_files.push(file_name);
        }
    });

    // remove old files for types that don't exist anymore
    for item in fs::read_dir(python_folder)?.flatten() {
        let os_file_name = item.file_name();
        let Some(file_name) = os_file_name.to_str() else {
            continue;
        };

        if python_files.iter().any(|item| item.as_str() == file_name) {
            continue;
        }

        fs::remove_file(python_folder.join(file_name))?;
    }

    python_mod.sort_unstable();
    fs::write(
        python_folder.join("mod.rs"),
        python_mod.into_iter().collect::<String>(),
    )?;

    let mut generated_planus =
        planus_codegen::generate_rust(&declarations)?.replace("RlBot", "RLBot");

    // remove all serde-related code
    generated_planus = generated_planus
        .replace("::serde::Serialize,", "")
        .replace("::serde::Deserialize,", "");

    fs::write(OUT_FILE, format_string(&generated_planus)?.as_bytes())?;

    class_inject::classes_to_lib_rs(class_names)?;
    pyi::generator(&declarations)?;

    Ok(())
}
