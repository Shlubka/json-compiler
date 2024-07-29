use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

pub trait Adding {
    fn adding(&self, long_string: &Mutex<String>);
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullJson {
    pub blocks: Vec<JsBlock>,
    pub arrows: Vec<Arrow>,
    pub x0: i32,
    pub y0: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsBlock {
    pub x: i32,
    pub y: i32,
    pub text: String,
    pub width: i32,
    pub height: i32,
    pub tupe: String,
    pub is_menu_block: bool,
    pub font_size: i32,
    pub text_height: i32,
    pub is_bold: bool,
    pub is_italic: bool,
    pub text_align: String,
    pub labels_position: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrow {
    pub start_index: usize,
    pub end_index: usize,
    pub start_connector_index: usize,
    pub end_connector_index: usize,
    pub nodes: Vec<Node>,
    pub counts: Vec<usize>,
}

impl Adding for JsBlock {
    fn adding(&self, long_string: &Mutex<String>) {
        let mut long_string = long_string.lock().unwrap();
        let mut main_json = from_str::<FullJson>(&long_string.clone()).unwrap();

        main_json.blocks.push(self.clone());
        let main_json_str = to_string_pretty(&main_json).unwrap();
        *long_string = main_json_str.clone();
    }
}

impl Adding for Arrow {
    fn adding(&self, long_string: &Mutex<String>) {
        let mut long_string = long_string.lock().unwrap();
        let mut main_json = from_str::<FullJson>(&long_string.clone()).unwrap();

        for i in main_json.blocks.iter() {}
    }
}
