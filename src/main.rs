#![warn(clippy::all, clippy::pedantic)]
use inquire::{Select, Text};
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use std::fs;
use std::io::{BufRead, Read};
use std::process::exit;
use std::sync::Mutex;
use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref LONG_STRING: Mutex<String> = Mutex::new(String::from(
        "{\"blocks\":[], \"arrows\": [],\"x0\": 0, \"y0\": 0}"
    ));
}

trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze(&self, path: &Path) -> Result<(), std::io::Error>;
}

trait Adding {
    fn adding(&self);
    //fn convert_to_json(&self, js: &String, path_to_json: &Path) -> Result<(), std::io::Error>;
}

#[derive(Default)]
struct Rust;

#[derive(Default)]
struct C;

#[derive(Default)]
struct CPlusPlus;

#[derive(Default)]
struct Java;

//json objs
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FullJson {
    blocks: Vec<JsBlock>,
    arrows: Vec<Arrow>,
    x0: i32,
    y0: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JsBlock {
    x: i32,
    y: i32,
    text: String,
    width: i32,
    height: i32,
    tupe: String,
    is_menu_block: bool,
    font_size: i32,
    text_height: i32,
    is_bold: bool,
    is_italic: bool,
    text_align: String,
    labels_position: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Arrow {
    start_index: usize,
    end_index: usize,
    start_connector_index: usize,
    end_connector_index: usize,
    nodes: Vec<Node>,
    counts: Vec<usize>,
}

impl Adding for JsBlock {
    fn adding(&self) {
        let mut long_string = LONG_STRING.lock().unwrap();
        let mut main_json = from_str::<FullJson>(&long_string.clone()).unwrap();

        main_json.blocks.push(self.clone());
        let main_json_str = to_string_pretty(&main_json).unwrap();
        *long_string = main_json_str.clone();
    }
    //fn convert_to_json(&self, js: &String, path_to_json: &Path) -> Result<(), std::io::Error> {
    //    todo!()
    //}
}

impl Adding for Arrow {
    fn adding(&self) {
        let mut long_string = LONG_STRING.lock().unwrap();
        let mut main_json = from_str::<FullJson>(&long_string.clone()).unwrap();

        for i in main_json.blocks.iter() {}
    }
}

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }

    fn analyze(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                //eprintln!("Failed to open file {}: {}", path.display(), error);
                return Err(error);
            }
        };
        let reader = BufReader::new(file);

        let mut mystack: Vec<char> = Vec::new();
        let mut external_func: Vec<String> = Vec::new();
        let mut block_stack: Vec<String> = Vec::new();
        let mut is_multiline_comment = false;
        //let mut is_rnter_point = false;
        let mut is_if = false;
        let mut is_else = false;
        let mut is_return = false;
        let mut x_global = 0;
        let mut y_global = 0;

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap_or_else(|_e| {
                // handle error here
                String::default()
            });

            let local_long_string = LONG_STRING.lock().unwrap();
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

            //look
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
                        //if is_rnter_point {
                        drop(local_long_string);
                        local_block.adding();
                    }
                    if mystack.len() == 0 && is_return {
                        is_return = false;
                    }
                    if block_name == "else" {
                        //println!("kjdfhjbsfkjdsnb");
                        x_global += 80
                    }
                    if block_name == "if" {
                        x_global -= 80
                    }
                    format!("exit block {block_name}")
                }
                s if s.contains("fn main") => {
                    //is_rnter_point = true;
                    mystack.push('{');
                    let block_name = s.split_whitespace().nth(1).unwrap_or("main");
                    block_stack.push(block_name.to_string());
                    y_global += 80;
                    local_block.text = String::from("начало");
                    local_block.tupe = String::from("Начало / конец");
                    drop(local_long_string);
                    local_block.adding();
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

                    //let local_long_string = LONG_STRING.unlock();
                    drop(local_long_string);
                    local_block.adding();
                    "fn".to_string()
                }
                s if s.contains("return") => {
                    is_return = true;
                    y_global += 80;
                    /*let local_block = JsBlock {
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
                    };*/
                    drop(local_long_string);
                    local_block.adding();
                    "exit fn".to_string()
                }
                s if external_func.iter().any(|kw| s.contains(kw)) => {
                    let func_name = s.split_whitespace().next().unwrap();
                    y_global += 80;
                    local_block.text = String::from(format!("{func_name}"));
                    local_block.tupe = String::from("Блок");
                    drop(local_long_string);
                    local_block.adding();
                    //let func_name = kw;
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
                    local_block.adding();
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
                    //println!("{y_global}");
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
                    local_block.adding();
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
                    /*let local_block = JsBlock {
                        x: x_global,
                        y: y_global,
                        text: format!("{}", line.trim_start().trim_end_matches(' ')).to_string(),
                        width: 100,
                        height: 30,
                        tupe: String::from("Блок"),
                        is_menu_block: false,
                        font_size: 14,
                        text_height: 14,
                        is_bold: false,
                        is_italic: false,
                        text_align: String::from("center"),
                        labels_position: 1,
                    };*/
                    local_block.text =
                        format!("{}", line.trim_start().trim_end_matches(' ')).to_string();
                    local_block.tupe = String::from("Блок");
                    local_block.y = y_global;
                    drop(local_long_string);
                    local_block.adding();
                    "action".to_string()
                }
            };

            println!("{i:>3} | {action:<17}| {:>2} | {line} ", mystack.len());
        }

        if mystack.len() > 0 {
            //println!("stack == 0");
            //return -1;
            exit(-1);
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

fn main() {
    println!("enter main");
    //let mut arrows = Arrow{};
    //-> Result<(), std::io::Error> {
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

    //let final_json =  Box::new(final_json);

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
    };

    let path = match file_path.as_str() {
        "default_file_path" => PathBuf::from(Text::new("Path").prompt().unwrap()),
        _ => PathBuf::from(file_path.clone()),
    };

    let selected_language = support_language
        .iter()
        .find(|x| x.get_name() == selected_language)
        .unwrap();
    //println!("Selected language: {}", selected_language.get_name());
    //println!("File path: {}", path.display());
    let _ = selected_language.analyze(&path);
    let long_string = LONG_STRING.lock().unwrap();

    let mut main_json = from_str::<FullJson>(&long_string).unwrap();

    let mut count = 0;

    //let mut y_acum = 0;
    //let mut x_acum = 0;

    let mut back_asum: [i32; 3] = [0, 0, 0]; // акамулятор для координат блока if
    let mut back_acum: [i32; 3] = [0, 0, 0]; // акамулятор для координат блока перед else

    for window in main_json.blocks.windows(2) {
        if window[0].text == "конец" {
            count += 1;
            continue;
        }

        println!("new duo");
        let average_x = (window[0].x + window[1].x) / 2;
        let average_y = (window[0].y + window[1].y) / 2;

        let mut arrow = Arrow {
            start_index: count,
            end_index: count + 1,
            start_connector_index: 2,
            end_connector_index: 0,
            nodes: vec![
                Node {
                    x: window[0].x,
                    y: window[0].y,
                },
                Node {
                    x: average_x,
                    y: average_y,
                },
                Node {
                    x: window[1].x,
                    y: window[1].y,
                },
            ],
            counts: vec![1, 1, 1],
        };

        if window[0].x == window[1].x {
            //println!("normal arrow");
            main_json.arrows.push(arrow);
        } else if window[0].tupe == "Условие" {
            //println!("Условие");
            arrow.start_connector_index = 1;
            back_asum = [window[0].x, window[0].y, count as i32];
            main_json.arrows.push(arrow);
        } else if window[1].y < window[0].y {
            back_acum = [window[0].x, window[0].y, count as i32];
            arrow.start_index = back_asum[2] as usize;
            arrow.end_index = count + 1;
            arrow.start_connector_index = 3;
            arrow.nodes = vec![
                Node {
                    x: back_asum[0],
                    y: back_asum[1],
                },
                Node {
                    x: average_x,
                    y: average_y,
                },
                Node {
                    x: window[1].x,
                    y: window[1].y,
                },
            ];
            main_json.arrows.push(arrow);
        } else if window[1].x > window[0].x {
            let arrow = Arrow {
                start_index: count,
                end_index: count + 1,
                start_connector_index: 2,
                end_connector_index: 0,
                nodes: vec![
                    Node {
                        x: window[0].x,
                        y: window[0].y,
                    },
                    Node {
                        x: window[0].x,
                        y: window[0].y + 35,
                    },
                    Node {
                        x: window[1].x,
                        y: window[0].y + 35,
                    },
                    Node {
                        x: window[1].x,
                        y: window[1].y,
                    },
                ],
                counts: vec![1, 1, 1, 1],
            };
            //let mut local_back_arrow = arrow.clone();
            main_json.arrows.push(arrow);
            let arrow = Arrow {
                start_index: back_acum[2] as usize,
                end_index: count + 1,
                start_connector_index: 2,
                end_connector_index: 0,
                nodes: vec![
                    Node {
                        x: back_acum[0],
                        y: back_acum[1],
                    },
                    Node {
                        x: back_acum[0],
                        y: back_acum[1] + 35,
                    },
                    Node {
                        x: window[1].x,
                        y: back_acum[1] + 35,
                    },
                    Node {
                        x: window[1].x,
                        y: window[1].y,
                    },
                ],
                counts: vec![1, 1, 1, 1],
            };
            main_json.arrows.push(arrow);
        }

        count += 1;
    }

    let long_string = to_string_pretty(&main_json).unwrap();
    //println!("{long_string}");
    fs::write("test.json", long_string.to_string().replace("tupe", "type")).expect("Error write");
    //Ok(())
}
