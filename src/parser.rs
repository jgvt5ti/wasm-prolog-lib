use crate::clause::{Goal, Literal, Program, Statement, Term};
use peg;

peg::parser! ( pub grammar prolog() for str {
    pub rule literal() -> Literal
        = _ s:$(['a'..='z']+) _ { Literal::Atom(String::from(s)) }
        / _ s:$(['A'..='Z']+) _ { Literal::Var(String::from(s))}

    pub rule term() -> Term
        = _ pred:$(['a'..='z']+) "(" arg:(literal() ** ",") ")" _ { Term { pred: String::from(pred), args: arg }}

    pub rule statement() -> Statement
        = _ cncl:term() ":-" asms:(term() ** ",") "." { Statement { conclusion: cncl, assumptions: asms } }
        / _ cncl:term() "." { Statement { conclusion: cncl, assumptions: Vec::new() } }

    pub rule program() -> Program
        = __ statements:(statement() ** __ ) { statements }

    pub rule goal() -> Goal
        = _ list:(term() ** ",") "." { Goal { list: list } }

    rule _ = [' ' | '\t']*
    rule __ = ['\n']*
});

#[test]
fn parse_test() {
    let a = "\np(a).\n\n\nq(X) :- r(X).";
    let t = prolog::program(a);
    println!("{:?}", t);
}
