mod mk_json_blocks;
use crate::mk_json_blocks::create_json_blocks;

mod lang_vec_stuf;
use crate::lang_vec_stuf::{Language, Rust, C};

use inquire::{Select, Text};
use std::fs;
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let lang = get_argument(&args, 1);
    let file_path = get_argument(&args, 2);
    let support_language: Vec<Box<dyn Language>> = vec![Box::new(Rust), Box::new(C)];

    let selected_language = select_language(&lang.unwrap(), &support_language);
    let path = PathBuf::from(file_path.unwrap_or_else(|| Text::new("Path").prompt().unwrap()));

    let analyzed_vector = selected_language.analyze_to_vec(&path);
    let final_string = String::from(create_json_blocks(analyzed_vector));

    let output_file_name = path.file_stem().unwrap().to_str().unwrap();
    let mut output_file_path = PathBuf::from("outfiles");
    output_file_path.push(format!("{}.json", output_file_name));
    fs::write(output_file_path, final_string).expect("Error write");
}

fn get_argument(args: &[String], index: usize) -> Option<String> {
    if args.len() > index {
        return Some(args[index].clone());
    }
    None
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
