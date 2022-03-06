use std::collections::HashMap;
use std::collections::HashSet;

pub type Substitution = HashMap<String, Term>;

pub fn composite_sbst(sbst1: &Substitution, sbst2: &Substitution) -> Substitution {
    let mut ans = sbst2.clone();
    for (k, v1) in sbst1 {
        ans.insert(k.clone(), v1.substitute(sbst2));
    }
    ans
}

pub fn composite_sbst_opt(
    sbst1: &Option<Substitution>,
    sbst2: &Option<Substitution>,
) -> Option<Substitution> {
    match (sbst1, sbst2) {
        (Some(s1), Some(s2)) => Some(composite_sbst(s1, s2)),
        _ => None,
    }
}

fn sbst_to_string(sbst: &Substitution) -> String {
    let mut ans = String::new();
    for (i, (k, v)) in sbst.iter().enumerate() {
        if i > 0 {
            ans += ", ";
        }
        ans += &(format!("{} = {}", &k, &(v.to_string())));
    }
    ans
}

// make a fresh variable
static mut VAR_COUNT: u32 = 0;

fn fresh_var() -> String {
    unsafe {
        VAR_COUNT += 1;
        format!("_{}", VAR_COUNT.to_string())
    }
}

fn make_fresh_sbst(st: &HashSet<String>) -> Substitution {
    st.iter()
        .map(|s| (s.clone(), Term::Var(fresh_var())))
        .collect()
}

#[derive(Debug, Clone)]
pub enum Term {
    Atom(String),
    Var(String),
    Pred(String, Vec<Term>),
}

impl Term {
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Term::Atom(_) => HashSet::new(),
            Term::Var(s) => HashSet::from([s.clone()]),
            Term::Pred(_, args) => args.iter().fold(HashSet::new(), |sum, x| {
                let mut sum = sum.clone();
                sum.extend(x.free_vars());
                sum
            }),
        }
    }

    pub fn substitute(&self, sbst: &Substitution) -> Term {
        match self {
            Term::Atom(_) => self.clone(),
            Term::Var(s) => match sbst.get(s) {
                Some(t) => t.clone(),
                None => self.clone(),
            },
            Term::Pred(name, args) => Term::Pred(
                name.clone(),
                args.iter().map(|s| s.substitute(sbst)).collect(),
            ),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Term::Atom(s) => s.clone(),
            Term::Var(s) => s.clone(),
            Term::Pred(name, args) => {
                let mut ans = name.clone() + "(";
                for (i, term) in (&args).iter().enumerate() {
                    if i > 0 {
                        ans += ", "
                    }
                    ans += &(term.to_string());
                }
                ans += ")";
                ans
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub conclusion: Term,
    pub assumptions: Vec<Term>,
}

impl Statement {
    pub fn free_vars(&self) -> HashSet<String> {
        let mut ret = self.conclusion.free_vars();
        for asm in self.assumptions.clone() {
            ret.extend(asm.free_vars());
        }
        ret
    }

    pub fn replace_newvar(&self) -> Statement {
        let vars = self.free_vars();
        let sbst = make_fresh_sbst(&vars);
        let nc = self.conclusion.substitute(&sbst);
        let na: Vec<Term> = self
            .assumptions
            .iter()
            .map(|t| t.substitute(&sbst))
            .collect();
        Statement {
            conclusion: nc,
            assumptions: na,
        }
    }
}

pub type Program = Vec<Statement>;

pub type Goal = Vec<Term>;

pub fn goals_free_vars(goal: &Goal) -> HashSet<String> {
    goal.iter().fold(HashSet::new(), |sum, x| {
        let mut sum = sum.clone();
        sum.extend(x.free_vars());
        sum
    })
}

pub fn goal_substitute(goal: &Goal, sbst: &Substitution) -> Goal {
    goal.into_iter().map(|term| term.substitute(sbst)).collect()
}

pub fn select_vars(sbst: &Substitution, varset: &HashSet<String>) -> Substitution {
    sbst.clone()
        .into_iter()
        .filter(|(k, _)| varset.contains(k))
        .collect()
}

#[derive(Debug, Clone)]
pub enum Answer {
    Valid(bool),
    Satisfiable(Option<Vec<Substitution>>),
}

impl Answer {
    pub fn to_string(&self) -> String {
        match self {
            Answer::Valid(true) => String::from("yes"),
            Answer::Valid(false) => String::from("no"),
            Answer::Satisfiable(Some(list)) => {
                let mut ans = String::new();
                for (i, sbst) in (&list).iter().enumerate() {
                    if i > 0 {
                        ans += "<br>";
                    }
                    ans += &sbst_to_string(sbst);
                }
                ans
            }
            Answer::Satisfiable(None) => String::from("Not Satisfiable!!"),
        }
    }
}
