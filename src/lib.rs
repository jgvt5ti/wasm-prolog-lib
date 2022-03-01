use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(s: &str) -> String {
    String::from(s) + "unko2"
}
