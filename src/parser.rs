use crate::syntax::{Goal, Program, Statement, Term};
use peg;

peg::parser! ( pub grammar prolog() for str {
    pub rule term() -> Term
        = _ pred:$(['a'..='z']+) "(" arg:(term() ** ",") ")" _ { Term::Pred(String::from(pred), arg) }
        / _ s:$(['a'..='z']+) _ { Term::Atom(String::from(s)) }
        / _ s:$(['A'..='Z']+) _ { Term::Var(String::from(s))}

    pub rule statement() -> Statement
        = _ cncl:term() ":-" asms:(term() ** ",") "." { Statement { conclusion: cncl, assumptions: asms } }
        / _ cncl:term() "." { Statement { conclusion: cncl, assumptions: Vec::new() } }

    pub rule program() -> Program
        = __ statements:(statement() ** __ ) { statements }

    pub rule goal() -> Goal
        = _ list:(term() ** ",") "." { list }

    rule _ = [' ' | '\t']*
    rule __ = ['\n']*
});
