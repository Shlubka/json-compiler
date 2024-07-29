use crate::mk_json_blocks::{Adding, FullJson, JsBlock};//, Node};
//use serde::{Deserialize, Serialize};
use serde_json::from_str;//, to_string_pretty};
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::{fs::File, io::BufReader, path::Path};

pub trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze(&self, path: &Path, long_string: Arc<Mutex<String>>) -> Result<(), std::io::Error>;
}

#[derive(Default)]
pub struct C;

impl Language for C {
    fn get_name(&self) -> &'static str {
        "C"
    }

    fn analyze(&self, path: &Path, long_string: Arc<Mutex<String>>) -> Result<(), std::io::Error> {
        todo!();
    }
}

#[derive(Default)]
pub struct CPlusPlus;

impl Language for CPlusPlus {
    fn get_name(&self) -> &'static str {
        "CPlusPlus"
    }

    fn analyze(&self, path: &Path, long_string: Arc<Mutex<String>>) -> Result<(), std::io::Error> {
        todo!();
    }
}

#[derive(Default)]
pub struct Java;

impl Language for Java {
    fn get_name(&self) -> &'static str {
        "Java"
    }

    fn analyze(&self, path: &Path, long_string: Arc<Mutex<String>>) -> Result<(), std::io::Error> {
        todo!();
    }
}

#[derive(Default)]
pub struct Rust;

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }

    fn analyze(&self, path: &Path, long_string: Arc<Mutex<String>>) -> Result<(), std::io::Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                return Err(error);
            }
        };
        let reader = BufReader::new(file);

        let mut mystack: Vec<char> = Vec::new();
        let mut external_func: Vec<String> = Vec::new();
        let mut block_stack: Vec<String> = Vec::new();
        let mut is_multiline_comment = false;
        let mut is_return = false;
        let mut x_global = 0;
        let mut y_global = 0;

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap_or_else(|_e| String::default());

            let local_long_string = long_string.lock().unwrap();
            let local_main_json = from_str::<FullJson>(&local_long_string.clone()).unwrap();

            if is_multiline_comment {
                if line.trim_start().starts_with("*/") {
                    is_multiline_comment = false;
                    drop(local_long_string);
                    continue;
                } else {
                    drop(local_long_string);
                    continue;
                }
            }

            let mut local_block = JsBlock {
                x: x_global,
                y: y_global,
                text: String::from("конец"),
                width: 100,
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

            let action = match line.as_str() {
                s if s.trim_start().starts_with("/*") => {
                    is_multiline_comment = true;
                    drop(local_long_string);
                    continue;
                }
                s if s.trim_start().starts_with("//") => continue,
                s if s.contains('}') => {
                    mystack.pop();
                    let block_name = block_stack.pop().unwrap_or("block".to_string());
                    if mystack.len() == 0 && !is_return {
                        y_global += 80;
                        drop(local_long_string);
                        local_block.adding(&long_string);
                    }
                    if mystack.len() == 0 && is_return {
                        is_return = false;
                    }
                    if block_name == "else" {
                        x_global += 80;
                        y_global += 80
                    }
                    if block_name == "if" {
                        x_global -= 80
                    }
                    format!("exit block {block_name}")
                }
                s if s.contains("fn main") => {
                    mystack.push('{');
                    let block_name = s.split_whitespace().nth(1).unwrap_or("main");
                    block_stack.push(block_name.to_string());
                    y_global += 80;
                    local_block.text = String::from("начало");
                    local_block.tupe = String::from("Начало / конец");
                    drop(local_long_string);
                    local_block.adding(&long_string);
                    "enter point".to_string()
                }
                s if s.contains("fn") => {
                    mystack.push('{');
                    y_global += 80;
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

                    local_block.text = String::from(external_func.last().unwrap().to_string());
                    local_block.tupe = String::from("Начало / конец");

                    drop(local_long_string);
                    local_block.adding(&long_string);
                    "fn".to_string()
                }
                s if s.contains("return") => {
                    is_return = true;
                    y_global += 80;
                    drop(local_long_string);
                    local_block.adding(&long_string);
                    "exit fn".to_string()
                }
                s if external_func.iter().any(|kw| s.contains(kw)) => {
                    let func_name = s.split_whitespace().next().unwrap();
                    y_global += 80;
                    local_block.text = String::from(format!("{func_name}"));
                    local_block.tupe = String::from("Блок");
                    drop(local_long_string);
                    local_block.adding(&long_string);
                    format!("call {func_name} ")
                }
                s if s.contains("let") || s.is_empty() => continue,
                s if s.contains("if") => {
                    y_global += 80;
                    mystack.push('{');
                    let block_name = "if".to_string();
                    block_stack.push(block_name);
                    local_block.text = String::from("if");
                    local_block.tupe = String::from("Условие");
                    x_global += x_global + 80;
                    drop(local_long_string);
                    local_block.adding(&long_string);
                    "if".to_string()
                }
                s if s.contains("else") => {
                    x_global += x_global - 80;
                    y_global = local_main_json
                        .blocks
                        .iter()
                        .filter(|block| block.tupe == "Условие")
                        .map(|block| block.y)
                        .max()
                        .unwrap_or(100);
                    mystack.push('{');
                    let block_name = "else".to_string();
                    block_stack.push(block_name);
                    "else".to_string()
                }
                s if s.contains("print") => {
                    y_global += 80;
                    local_block.text = String::from("вывод");
                    local_block.tupe = String::from("Ввод / вывод");
                    drop(local_long_string);
                    local_block.adding(&long_string);
                    "print".to_string()
                }
                s if s.contains('{') => {
                    mystack.push('{');
                    let block_name = s.split_whitespace().nth(0).unwrap_or("block");
                    block_stack.push(block_name.to_string());
                    drop(local_long_string);
                    "enter block".to_string()
                }
                _ => {
                    y_global += 80;
                    local_block.text =
                        format!("{}", line.trim_start().trim_end_matches(' ')).to_string();
                    local_block.tupe = String::from("Блок");
                    local_block.y = y_global;
                    drop(local_long_string);
                    local_block.adding(&long_string);
                    "action".to_string()
                }
            };

            println!("{i:>3} | {action:<17}| {:>2} | {line} ", mystack.len());
        }

        if mystack.len() > 0 {
            //println!("stack == 0");
            //return 1;
            //exit(-1);
        }

        Ok(())
    }
}
