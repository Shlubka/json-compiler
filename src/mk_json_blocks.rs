//use core::panicking::panic_const::panic_const_neg_overflow;
//use std::path::{Path, PathBuf};

//use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use crate::lang_vec_stuf::{BlockType, LocalVecBlock};

/*trait Adding {
    fn adding(&self);
}*/

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
/*impl Adding for JsBlock {
    fn adding(&self) {}
}

impl Adding for Arrow {
    fn adding(&self) {}
}*/

pub fn analyze(analyzed_vector: Vec<LocalVecBlock>) -> String {
    //-> PathBuf {
    //let mut cycle_acum_y = [0, 0]; //start cycle; end cycle
    //let mut x_max_min_acum = [0, 0]; // min max x for correct arrow adding for cycle
    //let mut is_cycle = false;
    let mut is_end_cycle = false;

    let mut local_full_blocks = FullJson {
        blocks: Vec::<JsBlock>::new(),
        arrows: Vec::<Arrow>::new(),
        x0: 0,
        y0: 0,
    };

    for i in analyzed_vector.iter() {
        let mut local_block = JsBlock {
            x: i.x,
            y: i.y,
            text: String::new(),
            width: 120,
            height: 60,
            tupe: String::from("Блок"),
            is_menu_block: false,
            font_size: 14,
            text_height: 14,
            is_bold: false,
            is_italic: false,
            text_align: String::new(),
            labels_position: 1,
        };

        if is_end_cycle {
            is_end_cycle = false;
            local_block.y += 10;
        }

        match i.r#type {
            BlockType::Start => {
                //println!("found start {} in vec {} {}", i.text, i.x, i.y);
                local_block.text = i.text.clone();
                local_block.tupe = String::from("Начало / конец");
            }
            BlockType::Condition => {
                //println!("found Condition {} in vec {} {}", i.text, i.x, i.y);
                local_block.text = i.text.clone();
                local_block.tupe = String::from("Условие");
            }
            BlockType::Actoin => {
                //println!("found {} in vec {} {}", i.text, i.y, i.x);
                local_block.text = i.text.clone();
            }
            BlockType::End => {
                //println!("found end in vec {} {} {}", i.y, i.x, i.text);
                match i.text == "cycle" {
                    false => {
                        local_block.tupe = String::from("Начало / конец");
                        local_block.text = match i.text.is_empty() {
                            false => i.text.clone(),
                            true => "Конец".to_string().clone(),
                        }
                    }
                    true => {
                        is_end_cycle = true;
                        local_block.tupe = String::from("Блок");
                        local_block.text = String::from("iter++")
                    }
                }
            }
            BlockType::Print => {
                //println!("found print in vec {} {}", i.y, i.x);
                local_block.tupe = String::from("Ввод / вывод");
                local_block.text = String::from("Вывод строки");
            }
            BlockType::Cycle => {
                //is_cycle = true;
                //cycle_acum_y[0] = i.y;
                //println!("found cycle in vec {} {}", i.y, i.x);
                local_block.tupe = String::from("Условие");
                local_block.text = i.text.to_string().clone();
            }
        }
        /*if is_cycle {
            if i.x > x_max_min_acum[0] {
                x_max_min_acum[0] = i.x;
            } else if i.x < x_max_min_acum[1] {
                x_max_min_acum[1] = i.x;
            }
        }*/
        local_full_blocks.blocks.push(local_block)
    }
    return to_string_pretty(&local_full_blocks).unwrap();
}
