use std::io::BufRead;
use std::{fs::File, io::BufReader, path::Path};

pub trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze_to_vec(&self, path: &Path) -> Vec<LocalVecBlock>;
}

#[derive(Debug, PartialEq)]
enum IfOrCycle {
    If,
    For,
    While,
    Loop,
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Start,
    End,
    Action,
    Print,
    Condition,
    Cycle,
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
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut state = AnalyzerState::new();

        for line in reader.lines() {
            let line = line.unwrap_or_else(|_e| String::default());
            if line.trim_start().starts_with("//") || line.starts_with("#") {
                continue;
            }
            state.process_line_c(&line);
        }

        if state.bracket_stack.len() > 0 {
            panic!("bracket_stack > 0");
        }

        state.return_vec
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

        let mut state = AnalyzerState::new();

        for line in reader.lines() {
            let line = line.unwrap_or_else(|_e| String::default());
            state.process_line_rs(&line);
        }

        if state.bracket_stack.len() > 0 {
            panic!("bracket_stack > 0");
        }

        state.return_vec
    }
}

struct AnalyzerState {
    bracket_stack: Vec<char>,
    external_func: Vec<String>,
    return_vec: Vec<LocalVecBlock>,
    block_stack: Vec<String>,
    is_multiline_comment: bool,
    is_return: bool,
    //is_if: i32,
    is_else: bool,
    is_cycle: bool,
    x_global: i32,
    y_global: i32,
    is_if_acum: Vec<[i32; 2]>,
    if_or_cycle_stack: Vec<IfOrCycle>,
}

impl AnalyzerState {
    fn new() -> Self {
        Self {
            bracket_stack: Vec::new(),
            external_func: Vec::new(),
            return_vec: Vec::new(),
            block_stack: Vec::new(),
            is_multiline_comment: false,
            is_return: false,
            //is_if: 0,
            is_else: false,
            is_cycle: false,
            x_global: 0,
            y_global: 0,
            //is_if_acum: [0, 0, 0],
            is_if_acum: Vec::<[i32; 2]>::new(),
            if_or_cycle_stack: Vec::<IfOrCycle>::new(),
        }
    }

    fn process_line_rs(&mut self, line: &str) {
        let mut local_vec_block = LocalVecBlock {
            r#type: BlockType::Action,
            text: String::new(),
            x: self.x_global,
            y: self.y_global,
        };

        if self.is_multiline_comment {
            if line.trim_start().starts_with("*/") {
                self.is_multiline_comment = false;
            }
            return;
        }

        if line.len() == 0 {
            return;
        }

        match line.trim_start() {
            s if s.starts_with('}') => self.handle_closing_bracket(&mut local_vec_block, false),
            s if s.starts_with("fn ") => self.handle_function(&mut local_vec_block, s),
            s if s.starts_with("return") => self.handle_return(&mut local_vec_block, s),
            s if self.external_func.iter().any(|kw| s.contains(kw)) => {
                self.handle_external_func(&mut local_vec_block, s)
            }
            s if s.starts_with("if") => self.handle_if(&mut local_vec_block, s),
            s if s.starts_with("else") => self.handle_else(&mut local_vec_block),
            s if s.contains("print") => self.handle_io(&mut local_vec_block, false),
            s if s.starts_with('{') && s.len() == 1 => return,
            s if s.starts_with("loop") => self.handle_loop(&mut local_vec_block),
            s if s.starts_with("for") => self.handle_for(&mut local_vec_block, s),
            s if s.starts_with("while") => self.handle_while(&mut local_vec_block, s),
            _ => self.handle_default(&mut local_vec_block, line),
        }

        self.return_vec.push(local_vec_block);
    }

    fn process_line_c(&mut self, line: &str) {
        let init_prefix = vec![
            "int", "void", "float", "double", "char", "short", "long", "unsigned", "signed",
            "const", "volatile",
        ];

        let memory_prefix = vec!["malloc", "calloc", "realloc", "free"];

        let out_of_func_init_prefix = vec!["struct", "enum", "union"];
        let mut out_of_func_init_prefix_flag = false;
        if out_of_func_init_prefix_flag {
            //if self.bracket_stack.last().unwrap().to_string() == "o" {
            if self.bracket_stack.len() == 0 {
                out_of_func_init_prefix_flag = false;
                self.return_vec.pop();
                return;
            }
        }

        if self.is_multiline_comment {
            if line.trim_start().starts_with("*/") {
                self.is_multiline_comment = false;
            }
            self.return_vec.pop();
            return;
        }

        if line.len() == 0 {
            //self.return_vec.pop();
            return;
        }

        let mut local_vec_block = LocalVecBlock {
            r#type: BlockType::Action,
            text: String::new(),
            x: self.x_global,
            y: self.y_global,
        };

        println!("{} {}", self.bracket_stack.len(), line);
        match line.trim_start() {
            s if s.starts_with("/*") => {
                self.is_multiline_comment = true;
                return;
            }
            s if s.starts_with("#") && self.bracket_stack.len() == 0 => {}
            s if s.starts_with('}') => {
                self.handle_closing_bracket(&mut local_vec_block, false) //out_of_func_init_prefix_flag)
            }
            s if s.starts_with("scanf") => self.handle_io(&mut local_vec_block, true),
            s if init_prefix.iter().any(|&prefix| s.starts_with(prefix)) => {
                if !s.contains(";") {
                    //println!("\nfound new func\n");
                    self.handle_function_c(&mut local_vec_block, s)
                } else {
                    return;
                }
            }
            s if memory_prefix.iter().any(|&prefix| s.contains(prefix)) => {
                self.handle_memory(&mut local_vec_block, s)
            }
            s if out_of_func_init_prefix
                .iter()
                .any(|&prefix| s.starts_with(prefix)) =>
            {
                out_of_func_init_prefix_flag = true;
                if self.bracket_stack.len() == 0 {
                    self.bracket_stack.push('{');
                }
            }
            s if s.starts_with("return") => self.handle_return(&mut local_vec_block, s),
            s if self.external_func.iter().any(|kw| s.contains(kw)) => {
                self.handle_external_func(&mut local_vec_block, s)
            }
            s if s.starts_with("if") => self.handle_if(&mut local_vec_block, s),
            s if s.starts_with("else") => self.handle_else(&mut local_vec_block),
            s if s.contains("print") => self.handle_io(&mut local_vec_block, false),
            s if s.starts_with('{') && s.len() == 1 => return,
            //s if s.starts_with("loop") => self.handle_loop(&mut local_vec_block),
            s if s.starts_with("for") => self.handle_for(&mut local_vec_block, s),
            s if s.starts_with("while") => self.handle_while(&mut local_vec_block, s),
            _ => self.handle_default(&mut local_vec_block, line),
        }

        self.return_vec.push(local_vec_block);
    }

    fn handle_closing_bracket(&mut self, block: &mut LocalVecBlock, out_of_fn: bool) {
        if out_of_fn {
            self.bracket_stack.pop();
            return;
        }
        if let Some(_) = self.bracket_stack.pop() {
            //block.text = "Конец".to_string();
            block.r#type = BlockType::End;
            self.y_global += 100;
            match self.if_or_cycle_stack.last().unwrap() {
                IfOrCycle::For => {}
                IfOrCycle::If => {}
                IfOrCycle::Loop => {}
                IfOrCycle::While => {}
            }
            if self.is_cycle {
                self.is_cycle = false;
                block.text = "cycle".to_string();
                //self.x_global -= 20;
                //block.r#type = BlockType::END_LOOP;
            } else if self.is_else {
                self.y_global = self.y_global.max(self.is_if_acum.last().unwrap()[1]);
                self.is_else = false;
                self.x_global += 100;
                return;
            } else if self.is_if_acum.len() > 0 && !self.is_else {
                //self.is_if_acum[2] = self.y_global;
                //self.is_if -= 1;
                //self.is_if_acum[2] -= 1;
                return;
            } else if self.is_return && self.bracket_stack.is_empty() {
                self.is_return = false;
                return;
            }
        } else {
            panic!("Unmatched closing bracket");
        }
    }

    fn handle_function(&mut self, block: &mut LocalVecBlock, line: &str) {
        block.r#type = BlockType::Start;
        self.bracket_stack.push('{');
        if line.contains("main") {
            self.block_stack.push("main".to_string());
            block.text = "Начало".to_string();
        } else {
            let func_name = line.split_whitespace().nth(1).unwrap().to_string();
            self.block_stack.push(func_name.clone());
            block.text = func_name;
        }
        self.x_global = 0;
        block.x = self.x_global;
        self.y_global += 100;
        block.y = self.y_global;
        self.y_global += 100;
    }
    fn handle_function_c(&mut self, block: &mut LocalVecBlock, line: &str) {
        //println!("\n fount new fn");
        if !line.contains(";") {
            block.r#type = BlockType::Start;
            self.bracket_stack.push('{');
            if line.contains("main") {
                self.block_stack.push("main".to_string());
                block.text = "Начало".to_string();
            } else {
                let func_name = line.split_whitespace().nth(1).unwrap().to_string();
                self.block_stack.push(func_name.clone());
                block.text = func_name;
            }
            self.x_global = 0;
            block.x = self.x_global;
            self.y_global += 100;
            block.y = self.y_global;
            self.y_global += 100;
        }
    }

    fn handle_return(&mut self, block: &mut LocalVecBlock, line: &str) {
        println!("{} {} {}", line, self.x_global, self.y_global);
        self.is_return = true;
        block.r#type = BlockType::End;
        block.text = line.trim().to_string();
    }

    fn handle_external_func(&mut self, block: &mut LocalVecBlock, line: &str) {
        let func_name = line.split_whitespace().next().unwrap().to_string();
        self.block_stack.push(func_name.clone());
        block.text = func_name;
        self.y_global += 100;
    }

    fn handle_if(&mut self, block: &mut LocalVecBlock, line: &str) {
        self.block_stack.push("if".to_string());
        self.bracket_stack.push('{');
        self.y_global += 100;
        self.is_if_acum.push([self.x_global - 100, self.y_global]);
        self.x_global += 100;
        block.text = line[2..line.len() - 1].to_string();
        block.r#type = BlockType::Condition;
    }

    fn handle_else(&mut self, _block: &mut LocalVecBlock) {
        self.block_stack.push("else".to_string());
        self.bracket_stack.push('{');
        self.is_else = true;
        self.x_global = self.is_if_acum.last().unwrap()[0];
        self.y_global = self.is_if_acum.last().unwrap()[1];
    }

    fn handle_io(&mut self, block: &mut LocalVecBlock, input: bool) {
        block.r#type = BlockType::Print;
        if input {
            block.text = String::from("Ввод")
        }
        self.y_global += 100;
    }

    fn handle_loop(&mut self, block: &mut LocalVecBlock) {
        self.block_stack.push("loop".to_string());
        self.bracket_stack.push('{');
        self.is_cycle = true;
        self.y_global += 100;
        block.text = "while true".to_string();
        block.r#type = BlockType::Cycle;
    }

    fn handle_for(&mut self, block: &mut LocalVecBlock, line: &str) {
        self.block_stack.push("for".to_string());
        self.bracket_stack.push('{');
        self.is_cycle = true;
        self.y_global += 100;
        block.text = line[..line.len() - 1].to_string();
        block.r#type = BlockType::Cycle;
    }

    fn handle_while(&mut self, block: &mut LocalVecBlock, line: &str) {
        self.block_stack.push("while".to_string());
        self.bracket_stack.push('{');
        self.is_cycle = true;
        self.y_global += 100;
        block.text = line[..line.len() - 1].to_string();
        block.r#type = BlockType::Cycle;
    }

    fn handle_memory(&mut self, block: &mut LocalVecBlock, line: &str) {
        self.y_global += 100;
        if line.starts_with("free") || line.starts_with("drop") {
            block.text = String::from("Очистка памяти")
        } else {
            block.text = String::from("Веделение памяти")
        }
    }

    fn handle_default(&mut self, block: &mut LocalVecBlock, line: &str) {
        self.y_global += 100;
        block.text = line.trim_start().to_string();
    }
}
