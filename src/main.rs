#![warn(clippy::all, clippy::pedantic)]
use inquire::{Select, Text};
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use std::fs;
use std::io::BufRead;
use std::sync::Mutex;
use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref LONG_STRING: Mutex<String> = Mutex::new(String::from(
        "{\"blocks\":[], \"arrows\": [],\"x0\": 0, \"y0\": 0}"
    ));
}

trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze(&self, path: &Path) -> Result<(), std::io::Error>;
}

trait Adding {
    fn adding(&self);
    //fn convert_to_json(&self, js: &String, path_to_json: &Path) -> Result<(), std::io::Error>;
}

pub enum TypesOfBlok {
    //Input: String = String::from("fdssds");
    StartEnd(String), //= "Начало / конец",
    InputOutput(String),
    Block(String),
    Condition(String),
    Cycle(String),
}

#[derive(Default)]
struct Rust;

#[derive(Default)]
struct C;

#[derive(Default)]
struct CPlusPlus;

#[derive(Default)]
struct Java;

//json objs
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FullJson {
    blocks: Vec<JsBlock>,
    arrows: Vec<Arrow>,
    x0: i32,
    y0: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JsBlock {
    x: i32,
    y: i32,
    text: String,
    width: i32,
    height: i32,
    tupe: String,
    is_menu_block: bool,
    font_size: i32,
    text_height: i32,
    is_bold: bool,
    is_italic: bool,
    text_align: String,
    labels_position: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Arrow {
    start_index: usize,
    end_index: usize,
    start_connector_index: usize,
    end_connector_index: usize,
    nodes: Vec<(i32, i32)>,
    counts: Vec<usize>,
}

impl Adding for JsBlock {
    fn adding(&self) {
        let mut long_string = LONG_STRING.lock().unwrap();
        let mut main_json = from_str::<FullJson>(&long_string.clone()).unwrap();

        main_json.blocks.push(self.clone());
        let main_json_str = to_string_pretty(&main_json).unwrap();
        *long_string = main_json_str.clone();
    }
    //fn convert_to_json(&self, js: &String, path_to_json: &Path) -> Result<(), std::io::Error> {
    //    todo!()
    //}
}

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }

    fn analyze(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Failed to open file {}: {}", path.display(), error);
                return Err(error);
            }
        };
        let reader = BufReader::new(file);

        let mut mystack: Vec<char> = Vec::new();
        let mut external_func: Vec<String> = Vec::new();
        let mut block_stack: Vec<String> = Vec::new();
        let mut is_multiline_comment = false;

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap_or_else(|_e| {
                // handle error here
                String::default()
            });

            if is_multiline_comment {
                if line.trim_start().starts_with("*/") {
                    is_multiline_comment = false;
                    continue;
                } else {
                    continue;
                }
            }

            let action = match line.as_str() {
                s if s.trim_start().starts_with("/*") => {
                    is_multiline_comment = true;
                    continue;
                }
                s if s.contains('}') => {
                    mystack.pop();
                    let block_name = block_stack.pop().unwrap_or("block".to_string());
                    format!("exit block {block_name}")
                }
                s if s.contains("fn main") => {
                    mystack.push('{');
                    let block_name = s.split_whitespace().nth(1).unwrap_or("main");
                    block_stack.push(block_name.to_string());
                    let local_block = JsBlock {
                        x: 0,
                        y: 0,
                        text: String::from("начало"),
                        width: 10,
                        height: 30,
                        tupe: String::from("Начало / конец"),
                        is_menu_block: false,
                        font_size: 14,
                        text_height: 14,
                        is_bold: false,
                        is_italic: false,
                        text_align: String::from("center"),
                        labels_position: 1,
                    };
                    local_block.adding();
                    "enter point".to_string()
                }
                s if s.contains("fn") => {
                    mystack.push('{');
                    let func_name = s
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .split('(')
                        .next()
                        .unwrap();
                    external_func.push(func_name.to_string());
                    let block_name = func_name.to_string();
                    block_stack.push(block_name);
                    let local_block = JsBlock {
                        x: 0,
                        y: 0,
                        text: String::from("начало"),
                        width: 10,
                        height: 30,
                        tupe: String::from(external_func.last().unwrap().to_string()),
                        is_menu_block: false,
                        font_size: 14,
                        text_height: 14,
                        is_bold: false,
                        is_italic: false,
                        text_align: String::from("center"),
                        labels_position: 1,
                    };
                    local_block.adding();
                    "fn".to_string()
                }
                s if s.contains("return") => "exit fn".to_string(),
                s if external_func.iter().any(|kw| s.contains(kw)) => {
                    let func_name = s.split_whitespace().nth(3).unwrap();
                    let local_block = JsBlock {
                        x: 0,
                        y: 0,
                        text: String::from("начало"),
                        width: 10,
                        height: 30,
                        tupe: String::from("call {func_name} "),
                        is_menu_block: false,
                        font_size: 14,
                        text_height: 14,
                        is_bold: false,
                        is_italic: false,
                        text_align: String::from("center"),
                        labels_position: 1,
                    };
                    local_block.adding();
                    //let func_name = kw;
                    format!("call {func_name} ")
                }
                s if s.contains("let") || s.is_empty() => continue,
                s if s.trim_start().starts_with("//") => continue,
                s if s.contains("if") => {
                    mystack.push('{');
                    let block_name = "if".to_string();
                    block_stack.push(block_name);
                    let local_block = JsBlock {
                        x: 0,
                        y: 0,
                        text: String::from("начало"),
                        width: 10,
                        height: 30,
                        tupe: String::from("call {func_name} "),
                        is_menu_block: false,
                        font_size: 14,
                        text_height: 14,
                        is_bold: false,
                        is_italic: false,
                        text_align: String::from("center"),
                        labels_position: 1,
                    };
                    local_block.adding();
                    "if".to_string()
                }
                s if s.contains("else") => {
                    mystack.push('{');
                    let block_name = "else".to_string();
                    block_stack.push(block_name);
                    "else".to_string()
                }
                s if s.contains('{') => {
                    mystack.push('{');
                    let block_name = s.split_whitespace().nth(0).unwrap_or("block");
                    block_stack.push(block_name.to_string());
                    "enter block".to_string()
                }
                _ => "action".to_string(),
            };

            println!("{i:>3} | {action:<17}| {:>2} | {line} ", mystack.len());
        }

        if mystack.is_empty() {
            println!("stack == 0");
        }

        Ok(())
    }
}

impl Language for C {
    fn get_name(&self) -> &'static str {
        "C"
    }

    fn analyze(&self, path: &Path) -> Result<(), std::io::Error> {
        todo!()
    }
}

impl Language for CPlusPlus {
    fn get_name(&self) -> &'static str {
        "C++"
    }

    fn analyze(&self, path: &Path) -> Result<(), std::io::Error> {
        todo!()
    }
}

impl Language for Java {
    fn get_name(&self) -> &'static str {
        "Java"
    }

    fn analyze(&self, path: &Path) -> Result<(), std::io::Error> {
        todo!()
    }
}

fn main() {
    //-> Result<(), std::io::Error> {
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
        Box::new(C),
        Box::new(CPlusPlus),
        Box::new(Java),
    ];

    //let final_json =  Box::new(final_json);

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
    }

    let path = match file_path.as_str() {
        "default_file_path" => PathBuf::from(Text::new("Path").prompt().unwrap()),
        _ => PathBuf::from(file_path.clone()),
    };

    let selected_language = support_language
        .iter()
        .find(|x| x.get_name() == selected_language)
        .unwrap();
    println!("Selected language: {}", selected_language.get_name());
    println!("File path: {}", path.display());
    let _ = selected_language.analyze(&path);
    let long_string = LONG_STRING.lock().unwrap();
    println!("{long_string}");
    fs::write("test.json", long_string.to_string()).expect("Error write");
    //Ok(())
}
