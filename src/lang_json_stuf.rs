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
        }

        if mystack.len() > 0 {
            //println!("stack == 0");
            //return 1;
            //exit(-1);
        }

        Ok(())
    }
}
