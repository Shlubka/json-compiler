//use inquire::ui::Key;
use inquire::{Select, Text};
use std::io::BufRead;
use std::{
    fs:: File,
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
            map.insert("as", "");
            map.insert("break", "");
            map.insert("const", "");
            map.insert("continue", "");
            map.insert("crate", "");
            map.insert("dyn", "");
            map.insert("else", "if/else");
            map.insert("enum", "");
            map.insert("extern", "");
            map.insert("false", "");
            map.insert("fn", "funk");
            map.insert("for", "");
            map.insert("if", "");
            map.insert("impl", "");
            map.insert("in", "");
            map.insert("let", "");
            map.insert("loop", "");
            map.insert("match", "");
            map.insert("mod", "");
            map.insert("move", "");
            map.insert("mut", "");
            map.insert("pub", "");
            map.insert("ref", "");
            map.insert("return", "exit");
            map.insert("Self", "");
            map.insert("self", "");
            map.insert("static", "");
            map.insert("struct", "");
            map.insert("super", "");
            map.insert("trait", "");
            map.insert("true", "");
            map.insert("type", "");
            map.insert("union", "");
            map.insert("unsafe", "");
            map.insert("use", "");
            map.insert("where", "");
            map.insert("while", "");
            map.insert("print!", "i/o oper");
            map.insert("println!", "i/o oper");
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

        for line in reader.lines() {
            let line = line?;
            match rust_keywords
                .values()
                .find(|&key| line.contains(key))
                .copied()
            {
                Some("fn") => print!("String contains a value fn from the HashMap"),
                Some("mod") => print!("String contains a value 'mod' from the HashMap"),
                Some("struct") => print!("String contains a value 'struct' from the HashMap"),
                Some(key) => print!("String contains a value '{}' from the HashMap", key),
                None => print!("String does not contain a value from the HashMap"),
            }
            println!("   {line}");
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
        //""другие языки
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported language",
        )),
    }
}
