use crate::eval::unify;
use crate::eval::*;
use crate::parser::prolog;
use wasm_bindgen::prelude::*;

mod eval;
mod parser;
mod syntax;

#[wasm_bindgen]
pub fn run(program: &str, goal: &str) -> String {
    match (prolog::program(program), prolog::goal(goal)) {
        (Ok(pr), Ok(gl)) => execute(pr, gl).to_string(),
        _ => String::from("error!"),
    }
}

#[test]
fn parse_test() {
    let a = "q(X, b)";
    let b = "q(a(b), Z)";
    let ta = prolog::term(a);
    let tb = prolog::term(b);
    match (ta, tb) {
        (Ok(term1), Ok(term2)) => println!("{:?}", unify(&term1, &term2)),
        _ => println!("unko"),
    }
}
