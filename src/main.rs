#![warn(clippy::all, clippy::pedantic)]
use inquire::{Select, Text};
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

#[derive(Default)]
struct Rust;

#[derive(Default)]
struct C;

#[derive(Default)]
struct CPlusPlus;

#[derive(Default)]
struct Java;

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }

    fn analyze(&self, path: &Path) -> Result<(), std::io::Error> {
        //use rust_fuzzy_search::fuzzy_search;
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Failed to open file {}: {}", path.display(), error);
                return Err(error);
            }
        };
        let reader = BufReader::new(file);

        let mut i = 1;
        //let mut bracket_c = 0;
        //let mut test;
        //let mut res: Vec<(&str, f32)>;
        let mut external_func: Vec<String> = Vec::new();
        for line in reader.lines() {
            let line = line?;     //переаисать эту хуйню через match
            if line.contains("fn main") { // создать 2 вектора, и рогнать их либо чеоез for либо
                                          // через iter
                print!("Line {i:>3} have enter point      ")
            } else if line.contains("fn") {
                print!("Line {i:>3} have 'fn'             ");
                let func_name = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .split('(')
                .next()
                .unwrap();
            external_func.push(func_name.to_string());
            } else if line.contains("return") {
                print!("Line {i:>3} have exit from fn     ")
            } else if let Some(external_func) = external_func.iter().find(|&kw| line.contains(kw)) {
                print!("Line {i:>3} have call of {:<7}  ", external_func)
            } else if line.contains("let") {
                i += 1;
                continue;
            } else if line.len() == 0 {
                i += 1;
                continue;
            } else if line.contains("if") {
                print!("Line {i:>3} have if               ")
            } else if line.contains("else") {
                print!("Line {i:>3} have else             ")
            }
            else {
                print!("Line {i:>3} have unnow pattern    ")
            }
            println!("|      {line}");
            i += 1;
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
    let support_language: Vec<Box<dyn Language>> = vec![
        Box::new(Rust),
        Box::new(C),
        Box::new(CPlusPlus),
        Box::new(Java),
    ];

    let selected_language: &str = Select::new(
        "Language?",
        support_language.iter().map(|x| x.get_name()).collect(),
    )
    .prompt()
    .unwrap();
    let path = PathBuf::from(Text::new("Path").prompt().unwrap());

    let selected_language = support_language
        .iter()
        .find(|x| x.get_name() == selected_language)
        .unwrap();
    println!("Selected language: {}", selected_language.get_name());
    println!("File path: {}", path.display());
    let _ = selected_language.analyze(&path);
    Ok(())
}
