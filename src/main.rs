#![warn(clippy::all, clippy::pedantic)]
use inquire::{Select, Text};
use std::ascii::AsciiExt;
use std::env;
//use serde_json::json;
//use rust_fuzzy_search::{fuzzy_compire};
use std::io::BufRead;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze(&self, path: &Path) -> Result<(), std::io::Error>;
}
/*
trait AddToJson {
    fn adding(&self) -> Result<(), std::io::Error>;
}
*/
#[derive(Default)]
struct Rust;

#[derive(Default)]
struct C;

#[derive(Default)]
struct CPlusPlus;

#[derive(Default)]
struct Java;
/*
//json objs
#[derive(Debug, Clone, PartialEq)]
struct Obj<'a> {
    x: i32,
    y: i32,
    text: &'a str,
    width: i32,
    height: i32,
    type_: &'a str,
    is_menu_block: bool,
    font_size: i32,
    text_height: i32,
    is_bold: bool,
    is_italic: bool,
    text_align: &'a str,
    labels_position: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arrow {
    pub start_index: usize,
    pub end_index: usize,
    pub start_connector_index: usize,
    pub end_connector_index: usize,
    pub nodes: Vec<(i32, i32)>,
    pub counts: Vec<usize>,
}

trait Visitor {
    fn visit_obj(&mut self, obj: &Obj) -> String;
    fn visit_arrow(&mut self, arrow: &Arrow) -> String;
}

struct JsonVisitor;

impl Visitor for JsonVisitor {
    fn visit_obj(&mut self, obj: &Obj) -> String {
        format!(r#"
        {{
            "x": {},
            "y": {},
            "text": "{}",
            "width": {},
            "height": {},
            "type": "{}",
            "is_menu_block": {},
            "font_size": {},
            "text_height": {},
            "is_bold": {},
            "is_italic": {},
            "text_align": "{}",
            "labels_position": {}
        }}
    "#, obj.x, obj.y, obj.text, obj.width, obj.height, obj.type_, obj.is_menu_block, obj.font_size, obj.text_height, obj.is_bold, obj.is_italic, obj.text_align, obj.labels_position)
    }

    fn visit_arrow(&mut self, arrow: &Arrow) -> String {
        format!(r#"
        {{
            "start_index": {},
            "end_index": {},
            "start_connector_index": {},
            "end_connector_index": {},
            "nodes": {},
            "counts": {}
        }}
    "#, arrow.start_index, arrow.end_index, arrow.start_connector_index, arrow.end_connector_index, json!(arrow.nodes), json!(arrow.counts))
    }
}

trait Accept {
    fn accept(&self, visitor: &mut dyn Visitor) -> String;
}

impl Accept for Obj<'static> {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_obj(self)
    }
}

impl Accept for Arrow {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_arrow(self)
    }
}


*/

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
                return Default::default();
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
                s if s.contains("}") => {
                    mystack.pop();
                    let block_name = block_stack.pop().unwrap_or("block".to_string());
                    format!("exit block {}", block_name)
                }
                s if s.contains("fn main") => {
                    mystack.push('{');
                    let block_name = s.split_whitespace().nth(1).unwrap_or("main");
                    block_stack.push(block_name.to_string());
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
                    "fn".to_string()
                }
                s if s.contains("return") => "exit fn".to_string(),
                s if external_func.iter().any(|kw| s.contains(kw)) => {
                    let func_name = s.split_whitespace().nth(3).unwrap();
                    //let func_name = kw;
                    format!("call {} ", func_name)
                }
                s if s.contains("let") || s.len() == 0 => continue,
                s if s.trim_start().starts_with("//") => continue,
                s if s.contains("if") => {
                    mystack.push('{');
                    let block_name = "if".to_string();
                    block_stack.push(block_name);
                    "if".to_string()
                }
                s if s.contains("else") => {
                    mystack.push('{');
                    let block_name = "else".to_string();
                    block_stack.push(block_name);
                    "else".to_string()
                }
                s if s.contains("{") => {
                    mystack.push('{');
                    let block_name = s.split_whitespace().nth(0).unwrap_or("block");
                    block_stack.push(block_name.to_string());
                    "enter block".to_string()
                }
                _ => "action".to_string(),
            };

            println!("{i:>3} | {action:<17}| {:>2} | {line} ", mystack.len());
        }

        if mystack.len() == 0 {
            println!("stack == 0")
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

fn main() -> Result<(), std::io::Error> {
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
    Ok(())
}
