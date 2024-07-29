#![warn(clippy::all, clippy::pedantic)]

mod mk_json_blocks;
use crate::mk_json_blocks::{FullJson, Node, Arrow};

mod lang_json_stuf;
use crate::lang_json_stuf::{Java, Rust, C, CPlusPlus, Language};

mod lang_vec_stuf;
use crate::lang_json_stuf::{Java, Rust, C, CPlusPlus, Language};

use inquire::{Select, Text};
use serde_json::{from_str, to_string_pretty};
use std::fs;
use std::sync::{Arc, Mutex};
use std::{
    env,
    path::PathBuf,
};

fn main() {
    let long_string = Arc::new(Mutex::new(String::from("{\"blocks\":[], \"arrows\": [],\"x0\": 0, \"y0\": 0}")));
    println!("enter main");
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
        /*Box::new(C),
        Box::new(CPlusPlus),
        Box::new(Java),*/
    ];

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

    let _ = selected_language.analyze(&path, long_string.clone());

    let long_string = long_string.lock().unwrap();
    let mut main_json = from_str::<FullJson>(&long_string).unwrap();

    let mut count = 0;
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
            main_json.arrows.push(arrow);
        } else if window[0].tupe == "Условие" {
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
    fs::write("test.json", long_string.to_string().replace("tupe", "type")).expect("Error write");
}
