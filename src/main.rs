#![warn(clippy::all, clippy::pedantic)]

mod mk_json_blocks;
use crate::mk_json_blocks::{analyze};

//mod lang_json_stuf;
//use crate::lang_json_stuf::{Java, Rust, C, CPlusPlus, Language};

mod lang_vec_stuf;
use crate::lang_vec_stuf::{Language, Rust};

use inquire::{Select, Text};
//use serde_json::{from_str, to_string_pretty};
use std::fs;
use std::{
    env,
    path::PathBuf,
};

enum MkPngScheme {
    ProgForU,
    Drawio,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lang = match args.len() > 1 {
        true => args[1].clone(),
        false => "default_lang".to_string(), // или любое другое значение по умолчанию
    };
    let file_path = match args.len() > 2 {
        true => args[2].clone(),
        false => "default_file_path".to_string(), // или любое другое значение по умолчанию
    };
    let support_language: Vec<Box<dyn Language>> = vec![
        Box::new(Rust),
        /*Box::new(C),
        Box::new(CPlusPlus),
        Box::new(Java),*/
    ];

    let selected_service = MkPngScheme::ProgForU;
    let selected_language: &str;
    match lang.to_lowercase().as_str() {
        "rust" => selected_language = "Rust",
        "java" => selected_language = "java",
        "c" => selected_language = "C",
        "cpp" | "c++" | "cplusplus" => selected_language = "CPlusPlus",
        _ => {
            selected_language = Select::new(
                "Language?",
                support_language.iter().map(|x| x.get_name()).collect(),
            )
            .prompt()
            .unwrap();
        }
    };

    let path = match file_path.as_str() {
        "default_file_path" => PathBuf::from(Text::new("Path").prompt().unwrap()),
        _ => PathBuf::from(file_path.clone()),
    };

    let selected_language = support_language
        .iter()
        .find(|x| x.get_name() == selected_language)
        .unwrap();

    // make vector for next analyze to needed format
    let analyzed_vector: Vec<lang_vec_stuf::LocalVecBlock> = selected_language.analyze_to_vec(&path);

    // analyze vec for make needed format
    let mut long_string = String::new();

    match selected_service {
        MkPngScheme::ProgForU => long_string = analyze(analyzed_vector),
        MkPngScheme::Drawio   => print!("todo!"),
    }
    /*let long_string: String = match selected_service {
        MkPngScheme::Drawio => {
            todo!()
        }
        MkPngScheme::ProgForU => {
            analyze(analyzed_vector)
        }
    };*/

    fs::write("test.json", long_string.replace("tupe", "type")).expect("Error write");
}
