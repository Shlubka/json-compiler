#![warn(clippy::all, clippy::pedantic)]
//use inquire::ui::Key;
use inquire::{Select, Text};
use std::io::BufRead;
use std::primitive;
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
        let rust_keywords = {
            let mut map = std::collections::HashMap::new();
            map.insert("println!", "i/o oper");
            map.insert("print!", "i/o oper");
            map.insert("fn main () {", "enter point");
            map.insert("fn", "func");
            map.insert("if", "if/else");
            map.insert("else", "if/else");
            map.insert("match", "if/else");
            map.insert("\n", "probel");
            //map.insert("}", "");
            //map.insert("]", "");
            //map.insert("}", "");
            map
        };

        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Failed to open file {}: {}", path.display(), error);
                return Err(error);
            }
        };
        let reader = BufReader::new(file);

        let mut i = 1;
        for line in reader.lines() {
            let line = line?;
            //if line == "}"{continue;}
            match rust_keywords//.keys().find(|&keys| line.contains(keys)).copied()
                .keys()
                .find(|&key| line.contains(key))
                .map(|key| rust_keywords.get(key).unwrap())
            {
                Some(keys) => match keys {
                    & "enter point" => print!("String {i:>width$} enter point  |", width=3),
                    & "i/o oper"    => print!("String {i:>width$} i/o oper     |", width=3),
                    & "if/else"     => print!("String {i:>width$} if/else      |", width=3),
                    & "func"        => print!("String {i:>width$} func         |", width=3),
                    //& "probel"      => break,
                    _               => print!("String {i:>width$} unknown key  |", width=3),

                }//print!("String {i} have a '{keys}'"),
                //Some(&"func")        => print!("goyda"),
                None                => print!("String {i:>width$} action       |", width=3),
            }
            println!("{line}");
            i += 1;
            /*let keyword = rust_keywords
                .keys()
                .find(|&key| line.contains(key))
                .copied();

            match keyword {
                Some(key) => match rust_keywords.get(key).unwrap() {
                    &"fn main" => print!("enter point                 "),
                    //&"fn main" => print!("enter point                 "),
                    _          => print!("String {i} have a '{}' ", key),
                },
                None           => print!("String {i} have action      "),
            }
            println!("   {line}");
            i += 1;*/
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
    let sellang = selected_language.get_name();
    println!("Selected language: {}", &sellang);
    println!("File path: {}", path.display());
    match sellang.as_ref() {
        "Rust" => Rust.analyze(&path),
        "C" => C.analyze(&path),
        "CPlusPlus" => CPlusPlus.analyze(&path),
        "Java" => Java.analyze(&path),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported language",
        )),
    }
}
