use crate::eval::*;
use crate::parser::prolog;
use wasm_bindgen::prelude::*;

mod eval;
mod parser;
mod syntax;

#[wasm_bindgen]
pub fn run(program: &str, goal: &str) -> String {
    match (prolog::program(program), prolog::goal(goal)) {
        (Ok(pr), Ok(gl)) => execute(&pr, &gl).to_string(),
        _ => String::from("Syntax Error"),
    }
}

#[test]
fn parse_test() {
    let goal_str = "f(a).";
    let prog_str = "f(X) :- g(X).\ng(b).";
    println!("{}", run(prog_str, goal_str));
}
