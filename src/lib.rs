use crate::eval::*;
use crate::parser::prolog;
use syntax::Substitution;
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
    let a = "q(X).";
    let b = "q(X) :- p(X).\np(a).";
    let ta = prolog::goal(a);
    let tb = prolog::program(b);
    let sbst = Substitution::new();
    match (ta, tb) {
        (Ok(term1), Ok(term2)) => println!("{:?}", dfs(&term2, term1, &sbst)),
        _ => println!("unko"),
    }
}
