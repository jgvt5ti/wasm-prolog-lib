#[derive(Debug, Clone)]
pub enum Literal {
    Atom(String),
    Var(String),
}

#[derive(Debug, Clone)]
pub struct Term {
    pub pred: String,
    pub args: Vec<Literal>,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub conclusion: Term,
    pub assumptions: Vec<Term>,
}

pub type Program = Vec<Statement>;

#[derive(Debug, Clone)]
pub struct Goal {
    pub list: Vec<Term>,
}
