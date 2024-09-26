mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x0: i32, x1: i32) -> i32 {
    x0 + x1
}
