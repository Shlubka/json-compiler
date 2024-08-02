use std::path::{Path, PathBuf};

//use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

trait Adding {
    fn adding(&self);
}

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
    }
}

impl Adding for Arrow {
    fn adding(&self) {
    }
}

pub fn analyze (analyzed_vector: Vec<String>) {//-> PathBuf {
    for i in analyzed_vector.iter() {
        match i.as_str() {
            "main"    => println!("found main in vec"),
            "if"      => println!("found if in vec"),
            "else"    => println!("found else in vec"),
            "loop"    => println!("found loop in vec"),
            "for"     => println!("found for in vec"),
            "while"   => println!("found while in vec"),
            "print"   => println!("found print in vec"),
            "action"  => println!("found action in vec"),
            "return"  => println!("found return in vec"),
            _         => println!("found {i} in vec")
        }
    }
}
