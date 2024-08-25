#![warn(clippy::all, clippy::pedantic)]

mod mk_json_blocks;
use crate::mk_json_blocks::analyze;

mod lang_vec_stuf;
use crate::lang_vec_stuf::{Language, Rust};

use inquire::{Select, Text};
use std::fs;
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let lang = get_language(&args);
    let file_path = get_file_path(&args);
    let support_language: Vec<Box<dyn Language>> = vec![Box::new(Rust)];

    let selected_language = select_language(&lang, &support_language);
    let path = get_path(&file_path);

    let analyzed_vector = selected_language.analyze_to_vec(&path);
    let long_string = String::from(analyze(analyzed_vector));

    // Get name and create output file.json with input file name
    let output_file_name = path.file_stem().unwrap().to_str().unwrap();
    let output_file_path = format!("{}.json", output_file_name);
    fs::write(output_file_path, long_string.replace("tupe", "type")).expect("Error write");
}

fn get_language(args: &[String]) -> String {
    match args.len() > 1 {
        true => args[1].clone(),
        false => "default_lang".to_string(),
    }
}

fn get_file_path(args: &[String]) -> String {
    match args.len() > 2 {
        true => args[2].clone(),
        false => "default_file_path".to_string(),
    }
}

fn select_language<'a>(
    lang: &str,
    support_language: &'a [Box<dyn Language>],
) -> &'a Box<dyn Language> {
    let selected_language: &str = match lang.to_lowercase().as_str() {
        "rust" => "Rust",
        "java" => "Java",
        "c" => "C",
        "cpp" | "c++" | "cplusplus" => "CPlusPlus",
        _ => Select::new(
            "Language?",
            support_language.iter().map(|x| x.get_name()).collect(),
        )
        .prompt()
        .unwrap(),
    };

    support_language
        .iter()
        .find(|x| x.get_name() == selected_language)
        .unwrap()
}

fn get_path(file_path: &str) -> PathBuf {
    match file_path {
        "default_file_path" => PathBuf::from(Text::new("Path").prompt().unwrap()),
        _ => PathBuf::from(file_path),
    }
}
