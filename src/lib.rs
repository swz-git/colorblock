#[macro_use]
extern crate lazy_static;

use std::{panic, str::FromStr};

use palette::{color_difference::EuclideanDistance, FromColor, Lab, Srgb};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    name: String,
    color: Srgb,
    bin: Vec<u8>,
}

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref BLOCK_TABLE: Vec<Block> = {
        let block_table_bin = include_bytes!("./block_table");
        bincode::deserialize(block_table_bin).unwrap()
    };
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    // Makes panics appear in js console
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn block_table() -> JsValue {
    serde_wasm_bindgen::to_value(&BLOCK_TABLE.clone()).unwrap()
}

#[wasm_bindgen]
pub fn color_search(hex_color_str: &str) -> Result<JsValue, JsValue> {
    let color = Srgb::from_str(hex_color_str).ok().ok_or("invalid color")?;

    let color_lab = Lab::from_color(color.into_format());

    let mut block_table = BLOCK_TABLE.clone();

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
