use std::io::BufRead;
use std::process::exit;
use std::{fs::File, io::BufReader, path::Path};

pub trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze_to_vec(&self, path: &Path) -> Vec<LocalVecBlock>;
}
#[derive(Debug)]
pub enum BlockType {
    Start,
    End,
    Actoin,
    Print,
    Condition,
    Cycle,
    // Добавьте другие возможные типы блоков здесь
}

pub struct LocalVecBlock {
    pub r#type: BlockType,
    pub text: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct C;

impl Language for C {
    fn get_name(&self) -> &'static str {
        "C"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<LocalVecBlock> {
        todo!();
    }
}

#[derive(Default)]
pub struct CPlusPlus;

impl Language for CPlusPlus {
    fn get_name(&self) -> &'static str {
        "CPlusPlus"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<LocalVecBlock> {
        todo!();
    }
}

#[derive(Default)]
pub struct Java;

impl Language for Java {
    fn get_name(&self) -> &'static str {
        "Java"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<LocalVecBlock> {
        todo!();
    }
}

#[derive(Default)]
pub struct Rust;

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<LocalVecBlock> {
        let file = File::open(path).unwrap();

        let reader = BufReader::new(file);

        let mut bracket_stack: Vec<char> = Vec::new(); // stack for [{(
        let mut external_func: Vec<String> = Vec::new(); // stack for external func
        let mut return_vec = Vec::new(); // vec for return from this mod
        let mut block_stack: Vec<String> = Vec::new(); // stack for looking for block
        let mut is_multiline_comment = false;
        let mut is_return = false;
        let mut is_if = false;
        let mut is_else = false;
        let mut is_cycle = false;
        let mut x_global = 0;
        let mut y_global = 0;
        let mut is_if_acum = [0, 0, 0];// x_global; y_global; max y in if/else arms

        for (i, line) in reader.lines().enumerate() {
        println!("len brst == {}", bracket_stack.len());
            let mut local_vec_block = LocalVecBlock {
                r#type: BlockType::Actoin,
                text: String::new(),
                x: x_global,
                y: y_global,
            };
            let line = line.unwrap_or_else(|_e| String::default());

            if is_multiline_comment {
                if line.trim_start().starts_with("*/") {
                    is_multiline_comment = false;
                    continue;
                } else {
                    continue;
                }
            }

            if line.len() == 0 {
                continue;
            }

            return_vec.push(match line {
                s if s.trim_start().starts_with("/*") => {
                    is_multiline_comment = true;
                    continue;
                }
                s if s.trim_start().starts_with("//") => continue,
                s if s.trim_start().starts_with('}') => {
                    if is_cycle == true {
                        local_vec_block.r#type = BlockType::End;
                        local_vec_block.text = String::from("cycle");
                        bracket_stack.pop();
                    } else if is_if == true {
                        is_if_acum[2] = y_global;
                        is_if = false;
                        x_global -= 100;
                        bracket_stack.pop();
                        continue;
                    } else if is_else == true {
                        y_global = is_if_acum[2];
                        is_else = false;
                        x_global += 100;
                        println!("\nshuyli?\n");
                        bracket_stack.pop();
                        continue;
                    } else if is_return == true && bracket_stack.len() == 0{
                        is_return = false;
                        //bracket_stack.pop();
                        continue;
                    }
                    if block_stack.len() == 0 {
                        local_vec_block.r#type = BlockType::End;
                        bracket_stack.pop();
                    }
                    local_vec_block.r#type = BlockType::End;
                    local_vec_block.text = String::from("Конец");
                    y_global += 100;
                    bracket_stack.pop();
                    local_vec_block
                }
                s if s.trim_start().starts_with("fn main") => {
                    println!("start main");
                    block_stack.push("main".to_string());
                    local_vec_block.r#type = BlockType::Start;
                    local_vec_block.text = String::from("Начало");
                    y_global += 100;
                    local_vec_block.y = y_global;
                    y_global += 100;
                    local_vec_block
                }
                s if s.trim_start().starts_with("fn ") => {
                    let local_str = s.clone().split_whitespace().nth(1).unwrap().to_string();
                    block_stack.push(local_str.to_string());
                    println!("start {}", local_str.clone());
                    local_vec_block.r#type = BlockType::Start;
                    local_vec_block.text = local_str.to_string().clone();
                    y_global += 100;
                    local_vec_block.y = y_global;
                    y_global += 100;
                    local_vec_block
                }
                s if s.trim_start().starts_with("return") => {
                    println!("return");
                    is_return = true;
                    y_global += 100;
                    local_vec_block.r#type = BlockType::End;
                    local_vec_block.text = String::from(s.trim_start().to_string().clone());
                    local_vec_block
                }
                s if external_func.iter().any(|kw| s.contains(kw)) => {
                    let func_name = s.split_whitespace().next().unwrap().to_string();
                    block_stack.push(func_name.clone());
                    local_vec_block.text = func_name.clone();
                    y_global += 100;
                    local_vec_block
                }
                s if s.trim_start().starts_with("let") || s.is_empty() => continue,
                s if s.trim_start().starts_with("if") => {
                    block_stack.push("if".to_string());
                    println!("start if");
                    is_if_acum = [x_global - 100, y_global + 100, 0];
                    is_if = true;
                    y_global += 100;
                    x_global += 100;
                    local_vec_block.text = String::from(s.trim_start()[2..s.trim_start().len()-1].to_string().clone());
                    local_vec_block.r#type = BlockType::Condition;
                    local_vec_block
                }
                s if s.trim_start().starts_with("else") => {
                    block_stack.push("else".to_string());
                    println!("start else");
                    is_else = true;
                    y_global = is_if_acum[1];
                    x_global = is_if_acum[0];
                    continue;
                }
                s if s.contains("print") => {
                    println!("print");
                    local_vec_block.r#type = BlockType::Print;
                    y_global += 100;
                    local_vec_block
                }

                s if s.trim_start().starts_with('{') && s.trim_start().len() == 1 => continue,

                s if s.trim_start().starts_with("loop") => {
                    block_stack.push("loop".to_string());
                    println!("loop");
                    is_cycle = true;
                    y_global += 100;
                    local_vec_block.text = String::from("while true");
                    local_vec_block.r#type = BlockType::Cycle;
                    local_vec_block
                }
                s if s.trim_start().starts_with("for") => {
                    block_stack.push("for".to_string());
                    println!("for");
                    is_cycle = true;
                    y_global += 100;
                    local_vec_block.text = String::from(s.trim_start()[0..s.trim_start().len() -1].to_string().clone());
                    local_vec_block.r#type = BlockType::Cycle;
                    local_vec_block
                }
                s if s.trim_start().starts_with("while") => {
                    block_stack.push("while".to_string());
                    println!("while");
                    is_cycle = true;
                    y_global += 100;
                    local_vec_block.text = String::from(s.trim_start()[0..s.trim_start().len() -1].to_string().clone());
                    local_vec_block.r#type = BlockType::Cycle;
                    local_vec_block
                }
                _ => {
                    println!("action");
                    y_global += 100;
                    local_vec_block.text = String::from(line.trim_start().to_string().clone());
                    local_vec_block
                }
            });
        }

        if bracket_stack.len() > 0 {
            println!("stack == 0");
            panic!("bracket_stack > 0")
        }

        println!("\n\n");
        return_vec
    }
}
