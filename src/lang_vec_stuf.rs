use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

pub trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze_to_vec(&self, path: &Path) -> Vec<LocalVecBlock>;
}
#[derive(Debug)]
pub enum BlockType {
    Start,
    End,
    Action,
    Print,
    Condition,
    Cycle,
    START_IF, //служебные
    END_IF,
    START_ELSE,
    END_ELSE,
    START_LOOP,
    END_LOOP,
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

    fn analyze_to_vec(&self, _path: &Path) -> Vec<LocalVecBlock> {
        todo!();
    }
}

#[derive(Default)]
pub struct CPlusPlus;

impl Language for CPlusPlus {
    fn get_name(&self) -> &'static str {
        "CPlusPlus"
    }

    fn analyze_to_vec(&self, _path: &Path) -> Vec<LocalVecBlock> {
        todo!();
    }
}

#[derive(Default)]
pub struct Java;

impl Language for Java {
    fn get_name(&self) -> &'static str {
        "Java"
    }

    fn analyze_to_vec(&self, _path: &Path) -> Vec<LocalVecBlock> {
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
        let mut is_if = 0;
        let mut is_else = false;
        let mut is_cycle = false;
        let mut x_global = 0;
        let mut y_global = 0;
        let mut is_if_acum = [0, 0, 0]; // x_global; y_global; max y in if/else arms
                                        //let mut len_count = 0; // count for counting len for create link in sheme

        for (_, line) in reader.lines().enumerate() {
            let mut local_vec_block = LocalVecBlock {
                r#type: BlockType::Action,
                text: String::new(),
                x: x_global,
                y: y_global,
            };
            let line = line.unwrap_or_else(|_e| String::default());
            println!(
                "len brst == {} line = {}",
                bracket_stack.len(),
                line.clone()
            );

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

            //if is_if or is_else

            //println!("x_global == {x_global} y_global == {y_global}   ");
            match line.trim_start() {
                s if s.starts_with('}') => {
                    if let Some(_) = bracket_stack.pop() {
                        local_vec_block.text = "Конец".to_string();
                        local_vec_block.r#type = BlockType::End;
                        y_global += 100;
                        if is_cycle {
                            is_cycle = false;
                            local_vec_block.text = "cycle".to_string();
                        } else if is_else {
                            y_global = y_global.max(is_if_acum[2]);
                            is_else = false;
                            x_global += 100;
                            continue;
                        } else if is_if > 0 && !is_else {
                            is_if_acum[2] = y_global;
                            is_if -= 1;
                            continue;
                        } else if is_return && bracket_stack.is_empty() {
                            is_return = false;
                            continue;
                        }
                    } else {
                        panic!("Unmatched closing bracket");
                    }
                }
                s if s.starts_with("fn ") => {
                    local_vec_block.r#type = BlockType::Start;
                    bracket_stack.push('{');
                    if s.contains("main") {
                        block_stack.push("main".to_string());
                        local_vec_block.text = "Начало".to_string();
                    } else {
                        let func_name = s.split_whitespace().nth(1).unwrap().to_string();
                        block_stack.push(func_name.clone());
                        local_vec_block.text = func_name;
                    }
                    y_global += 100;
                    local_vec_block.y = y_global;
                    y_global += 100;
                }
                s if s.starts_with("return") => {
                    is_return = true;
                    //y_global += 100;
                    local_vec_block.r#type = BlockType::End;
                    local_vec_block.text = s.to_string();
                }
                s if external_func.iter().any(|kw| s.contains(kw)) => {
                    let func_name = s.split_whitespace().next().unwrap().to_string();
                    block_stack.push(func_name.clone());
                    local_vec_block.text = func_name;
                    y_global += 100;
                }
                s if s.starts_with("if") => {
                    block_stack.push("if".to_string());
                    bracket_stack.push('{');
                    y_global += 100;
                    is_if_acum = [x_global - 100, y_global, 0];
                    is_if += 1;
                    x_global += 100;
                    local_vec_block.text = s[2..s.len() - 1].to_string();
                    local_vec_block.r#type = BlockType::Condition;
                }
                s if s.starts_with("else") => {
                    block_stack.push("else".to_string());
                    bracket_stack.push('{');
                    is_else = true;
                    x_global = is_if_acum[0];
                    y_global = is_if_acum[1];
                    continue;
                }
                s if s.contains("print") => {
                    local_vec_block.r#type = BlockType::Print;
                    y_global += 100;
                }
                s if s.starts_with('{') && s.len() == 1 => continue,
                s if s.starts_with("loop") => {
                    block_stack.push("loop".to_string());
                    bracket_stack.push('{');
                    is_cycle = true;
                    y_global += 100;
                    local_vec_block.text = "while true".to_string();
                    local_vec_block.r#type = BlockType::Cycle;
                }
                s if s.starts_with("for") => {
                    block_stack.push("for".to_string());
                    bracket_stack.push('{');
                    is_cycle = true;
                    y_global += 100;
                    local_vec_block.text = s[..s.len() - 1].to_string();
                    local_vec_block.r#type = BlockType::Cycle;
                }
                s if s.starts_with("while") => {
                    block_stack.push("while".to_string());
                    bracket_stack.push('{');
                    is_cycle = true;
                    y_global += 100;
                    local_vec_block.text = s[..s.len() - 1].to_string();
                    local_vec_block.r#type = BlockType::Cycle;
                }
                _ => {
                    y_global += 100;
                    local_vec_block.text = line.to_string();
                }
            }

            return_vec.push(local_vec_block);
        }

        if bracket_stack.len() > 0 {
            println!("stack != 0");
            panic!("bracket_stack > 0")
        }

        println!("\n\n");
        return_vec
    }
}
