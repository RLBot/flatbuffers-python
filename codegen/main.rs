use eyre::ContextCompat;
use planus_types::{ast::IntegerType, intermediate::DeclarationKind};
use std::{env::set_current_dir, fs, path::Path};

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

    // generate custom code
    for (path, item) in &declarations.declarations {
        let item_name = path.0.last().unwrap().as_str();
        let mut file_name = camel_to_snake(item_name);

        let file_contents = match &item.kind {
            DeclarationKind::Table(info) => {
                let bind_gen =
                    TableBindGenerator::new(item_name, &info.fields, &declarations.declarations);
                bind_gen.generate_binds()
            }
            DeclarationKind::Struct(info) => {
                let bind_gen =
                    StructBindGenerator::new(item_name, &info.fields, &declarations.declarations);
                bind_gen.generate_binds()
            }
            DeclarationKind::Enum(info) => {
                let bind_gen = EnumBindGenerator::new(item_name, &info.variants);
                bind_gen.generate_binds()
            }
            DeclarationKind::Union(info) => {
                let bind_gen = UnionBindGenerator::new(item_name, &info.variants);
                bind_gen.generate_binds()
            }
            DeclarationKind::RpcService(_) => unimplemented!(),
        };

        class_names.push(item_name);
        python_mod.push(
            ["mod ", &file_name, ";\n", "pub use ", &file_name, "::*;\n"]
                .into_iter()
                .collect(),
        );
        file_name.push_str(".rs");

        fs::write(python_folder.join(&file_name), file_contents.join("\n"))?;
        python_files.push(file_name);
    }

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

    fs::write(OUT_FILE, generated_planus.as_bytes())?;

    class_inject::classes_to_lib_rs(class_names)?;
    pyi::generator(&declarations.declarations)?;

    Ok(())
}
