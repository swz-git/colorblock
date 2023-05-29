use std::{error::Error, str::FromStr};

use palette::{
    color_difference::{self, Ciede2000, EuclideanDistance},
    rgb::Rgb,
    white_point::D65,
    FromColor, Lab, Srgb,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    name: String,
    color: Srgb,
    bin: Vec<u8>,
}

// const fn get_block_table() -> Vec<Block> {
//     let textures = read_dir("./defaultpack/assets/minecraft/textures/block");

//     todo!()
// }

// const BLOCK_TABLE: Vec<Block> = gen_block_table();

fn get_block_table() -> Vec<Block> {
    let block_table_bin = include_bytes!("./block_table");
    bincode::deserialize(block_table_bin).unwrap()
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn block_table() -> JsValue {
    serde_wasm_bindgen::to_value(&get_block_table()).unwrap()
}

#[wasm_bindgen]
pub fn color_search(hex_color_str: &str) -> Result<JsValue, JsValue> {
    let color = Srgb::from_str(hex_color_str).ok().ok_or("invalid color")?;

    let color_lab = Lab::from_color(color.into_format());

    let mut block_table = get_block_table();

    block_table.sort_by(|x, y| {
        Lab::from_color(x.color)
            .distance(color_lab)
            .partial_cmp(&Lab::from_color(y.color).distance(color_lab))
            .unwrap()
    });

    Ok(serde_wasm_bindgen::to_value(&block_table).unwrap())
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
