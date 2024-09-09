use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use crate::lang_vec_stuf::{BlockType, LocalVecBlock};

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
    r#type: String,
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

pub fn create_json_blocks(analyzed_vector: Vec<LocalVecBlock>) -> String {
    println!();
    //-> PathBuf {
    let mut cycle_acum: [i32; 3] = [0, 0, 0]; //start cycle; end cycle
    let mut x_max_min_acum = [0, -100]; //max min x for correct arrow adding for cycle
    let mut is_cycle = false;
    let mut is_end_cycle = false;
    // let mut if_else_acum = [0, 0, 0]; //x, y, ? for loking if else coord
    //let mut previos_coord: [i32; 3] = [0, 0, 0];

    let mut local_full_blocks = FullJson {
        blocks: Vec::<JsBlock>::new(),
        arrows: Vec::<Arrow>::new(),
        x0: 0,
        y0: 0,
    };

    let mut iterator = 0;
    for i in analyzed_vector.iter() {
        let mut local_arrow = Arrow {
            start_index: iterator,
            end_index: iterator + 1,
            start_connector_index: 2,
            end_connector_index: 0,
            nodes: Vec::<Node>::new(),
            counts: vec![1, 1, 1],
        };

        let mut local_block = JsBlock {
            x: i.x,
            y: i.y,
            text: String::new(),
            width: 120,
            height: 60,
            r#type: String::from("Блок"),
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

        if is_cycle {
            println!("is cycle");
            println!(
                "{} {} {} {}",
                i.x, i.y, x_max_min_acum[0], x_max_min_acum[1]
            );
            if i.x > x_max_min_acum[0] {
                x_max_min_acum[0] = i.x;
                println!("more")
            }
            if i.x < x_max_min_acum[1] {
                x_max_min_acum[1] = i.x;
                println!("not more")
            }
        }

        let mut local_text = String::new();
        if i.text.len() > 16 {
            println!("enter");
            let mut chars = i.text.chars();
            let mid = chars.clone().count() / 2;
            let first_half: String = chars.by_ref().take(mid).collect();
            let second_half: String = chars.collect();
            local_text = format!("{}\n{}", first_half, second_half);
        }

        match i.r#type {
            BlockType::Start => {
                println!("found start {} in vec {} {}", i.text, i.x, i.y);
                local_block.text = i.text.clone();
                local_block.r#type = String::from("Начало / конец");
            }
            BlockType::Condition => {
                println!("found Condition {} in vec {} {}", i.text, i.x, i.y);
                local_block.text = i.text.clone();
                local_block.r#type = String::from("Условие");
            }
            BlockType::Action => {
                println!("found {} in vec {} {}", i.text, i.y, i.x);
                local_block.text = match local_text.is_empty() {
                    true => i.text.clone(),
                    false => local_text,
                }
            }
            BlockType::End => {
                println!("found end in vec {} {} {}", i.y, i.x, i.text);
                match i.text == "cycle" {
                    false => {
                        local_block.r#type = String::from("Начало / конец");
                        local_block.text = match i.text.is_empty() {
                            false => i.text.clone(),
                            true => "Конец".to_string().clone(),
                        }
                    }
                    true => {
                        //is_end_cycle = true;
                        is_cycle = false;
                        local_block.r#type = String::from("Блок");
                        local_block.text = String::from("iter++");

                        let value = vec![
                            Node {
                                x: i.x,
                                y: i.y - 30,
                            },
                            Node { x: i.x, y: i.y },
                            Node {
                                x: x_max_min_acum[1],
                                y: i.y,
                            },
                            Node {
                                x: x_max_min_acum[1],
                                y: cycle_acum[0] - 20,
                            },
                            Node {
                                x: cycle_acum[0],
                                y: cycle_acum[1] - 50,
                            },
                            Node {
                                x: cycle_acum[0],
                                y: cycle_acum[1],
                            },
                        ];
                        let local_arrow_local = Arrow {
                            start_index: iterator,
                            end_index: cycle_acum[2] as usize,
                            start_connector_index: 3,
                            end_connector_index: 0,
                            nodes: Vec::from(value),
                            counts: vec![1, 1, 1, 1, 1],
                        };
                        local_full_blocks.arrows.push(local_arrow_local);
                        local_arrow.start_index = cycle_acum[2] as usize;
                        local_arrow.end_index = iterator + 1;
                        local_arrow.start_connector_index = 1;
                        local_arrow.end_connector_index = 0;
                        local_arrow.nodes.extend([
                            Node {
                                x: x_max_min_acum[0] + 30,
                                y: cycle_acum[1],
                            },
                            Node {
                                x: x_max_min_acum[0] + 30,
                                y: i.y + 45,
                            },
                            Node {
                                x: i.x,
                                y: i.y + 45,
                            },
                        ]);
                    }
                }
            }
            BlockType::Print => {
                println!("found print in vec {} {}", i.y, i.x);
                local_block.r#type = String::from("Ввод / вывод");
                if i.text.is_empty() {
                    local_block.text = String::from("Вывод строки");
                } else {
                    local_block.text = String::from(i.text.as_str());
                }
            }
            BlockType::Cycle => {
                is_cycle = true;
                cycle_acum = [i.x, i.y, (iterator) as i32];
                println!("found cycle in vec {} {}", i.y, i.x);
                local_block.r#type = String::from("Условие");
                local_block.text = i.text.to_string().clone();
            }
        }
        local_full_blocks.arrows.push(local_arrow);
        local_full_blocks.blocks.push(local_block);
        iterator += 1;
    }
    local_full_blocks.arrows.pop();
    return to_string_pretty(&local_full_blocks).unwrap();
}
