use crate::parser::prolog;
use wasm_bindgen::prelude::*;
mod clause;
mod parser;

#[wasm_bindgen]
pub fn run(s: &str) -> String {
    String::from(s) + "unko"
}
