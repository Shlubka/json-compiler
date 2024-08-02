use std::io::BufRead;
use std::process::exit;
use std::{fs::File, io::BufReader, path::Path};

pub trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze_to_vec(&self, path: &Path) -> Vec<String>;
}

pub struct local_vec_block {
    tupe: String,
    text: String,
    x: i32,
    y: i32
}

#[derive(Default)]
pub struct C;

impl Language for C {
    fn get_name(&self) -> &'static str {
        "C"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<String> {
        todo!();
    }
}

#[derive(Default)]
pub struct CPlusPlus;

impl Language for CPlusPlus {
    fn get_name(&self) -> &'static str {
        "CPlusPlus"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<String> {
        todo!();
    }
}

#[derive(Default)]
pub struct Java;

impl Language for Java {
    fn get_name(&self) -> &'static str {
        "Java"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<String> {
        todo!();
    }
}

#[derive(Default)]
pub struct Rust;

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }

    fn analyze_to_vec(&self, path: &Path) -> Vec<String> {
        let file = File::open(path).unwrap();
        /*let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                return Err(error);
            }
        };*/

        let reader = BufReader::new(file);

        let mut bracket_stack: Vec<char> = Vec::new(); // stack for [{(
        let mut external_func: Vec<String> = Vec::new(); // stack for external func
        let mut return_vec = Vec::new(); // vec for return from this mod
        let mut block_stack: Vec<&str> = Vec::new(); // stack for looking for block
        let mut is_multiline_comment = false;
        let mut is_return = false;
        let mut is_if = false;
        let mut is_else = false;
        let mut x_global = 0;
        let mut y_global = 0;

        for (i, line) in reader.lines().enumerate() {
            local_vec_block {
                tupe: String::from("action"),
                text: String::new(),
                x: x_global,
                y: y_global,
            };
            let line = line.unwrap_or_else(|_e| String::default());

            //print!("{: <70}     |", line);

            if is_multiline_comment {
                if line.trim_start().starts_with("*/") {
                    is_multiline_comment = false;
                    //drop(local_long_string);
                    continue;
                } else {
                    //drop(local_long_string);
                    continue;
                }
            }

            if line.len() == 0 {continue;}

            return_vec.push(match line {
                s if s.trim_start().starts_with("/*") => {
                    is_multiline_comment = true;
                    //drop(local_long_string);
                    continue;
                }
                s if s.trim_start().starts_with("//") => continue,
                s if s.trim_start().starts_with('}') => {
                    //println!("\n{}\n", block_stack.len());
                    if block_stack.len() == 0 {
                        panic!("unopened bracket")
                    }
                    //return_vec
                    let local_end = format!("end {}", block_stack.last().unwrap().to_string().clone());
                    block_stack.pop();
                    local_end
                    //continue;
                    /*match block_stack.last().unwrap().to_string().as_str() {
                        "if" => {
                            println!("end if");
                            bracket_stack.pop();
                            continue;
                        }
                        "else" => {
                            println!("end else");
                            bracket_stack.pop();
                            continue;
                        }
                        "loop" => {
                            println!("end loop");
                            bracket_stack.pop();
                            continue;
                        }
                        "for" => {
                            println!("end for");
                            bracket_stack.pop();
                            continue;
                        }
                        "while" => {
                            println!("end while");
                            bracket_stack.pop();
                            continue;
                        }
                        "main" => {
                            println!("end main");
                            bracket_stack.pop();
                            continue;
                        }
                        _ => {
                            println!("idn");
                            continue;
                        }
                    }*/
                }
                s if s.trim_start().starts_with("fn main") => {
                    println!("start main");
                    //.push("main");
                    block_stack.push("main");
                    "main".to_string()
                }
                s if s.trim_start().starts_with("fn") => {
                    block_stack.push("huy");
                    println!("start ext fn");
                    "fn".to_string()
                }
                s if s.trim_start().starts_with("return") => {
                    println!("return");
                    "return".to_string()
                }
                s if external_func.iter().any(|kw| s.contains(kw)) => {
                    let func_name = s.split_whitespace().next().unwrap().to_string();
                    let static_func_name = Box::leak(func_name.into_boxed_str());
                    block_stack.push(static_func_name);
                    format!("call {static_func_name} ")
                }
                s if s.trim_start().starts_with("let") || s.is_empty() => continue,
                s if s.trim_start().starts_with("if") => {
                    block_stack.push("if");
                    println!("start if");
                    "if".to_string()
                }
                s if s.trim_start().starts_with("else") => {
                    block_stack.push("else");
                    println!("start else");
                    "else".to_string()
                }
                s if s.trim_start().starts_with("print") => {
                    println!("print");
                    "print".to_string()
                }
                s if s.trim_start().starts_with('{') => continue,


                s if s.trim_start().starts_with("loop") => {
                    block_stack.push("loop");
                    println!("loop");
                    "loop".to_string()
                }
                s if s.trim_start().starts_with("for") => {
                    block_stack.push("for");
                    println!("for");
                    "for".to_string()
                }
                s if s.trim_start().starts_with("while") => {
                    block_stack.push("while");
                    println!("while");
                    "while".to_string()
                }

                _ => {
                    println!("action");
                    "action".to_string()
                }
            })

            //println!("{i:>3} | {action:<17}| {:>2} | {line} ", mystack.len());
        }

        if bracket_stack.len() > 0 {
            println!("stack == 0");
            panic!("bracket_stack > 0")
        }

        println!("\n\n");
        return_vec
    }
}
