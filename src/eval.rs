use crate::syntax::*;
use std::collections::HashSet;
use std::iter::zip;

pub fn execute(mut program: Program, mut goal: Goal) -> Answer {
    program.pop();
    goal.pop();
    while !goal.is_empty() {}
    Answer::Valid(true)
}

pub fn unify(t1: &Term, t2: &Term) -> Option<Substitution> {
    let t1 = t1.clone();
    let t2 = t2.clone();
    match (t1, t2) {
        (Term::Atom(s1), Term::Atom(s2)) if s1 == s2 => Some(Substitution::new()),
        (Term::Var(s1), Term::Var(s2)) if s1 == s2 => Some(Substitution::new()),
        (Term::Var(s1), s2) if !s2.free_vars().contains(&s1) => {
            Some(Substitution::from([(s1, s2)]))
        }
        (s1, Term::Var(s2)) if !s1.free_vars().contains(&s2) => {
            Some(Substitution::from([(s2, s1)]))
        }
        (Term::Pred(p1, ls1), Term::Pred(p2, ls2)) if p1 == p2 => {
            unify_list(zip(ls1, ls2).collect())
        }
        _ => None,
    }
}

fn unify_list(mut ls: Vec<(Term, Term)>) -> Option<Substitution> {
    match ls.pop() {
        Some((s1, s2)) => match unify(&s1, &s2) {
            Some(sbst) => {
                let acc = unify_list(
                    ls.iter()
                        .map(|(t1, t2)| (t1.substitute(&sbst), t2.substitute(&sbst)))
                        .collect(),
                );
                composite_sbst_opt(&Some(sbst), &acc)
            }
            None => None,
        },
        None => Some(Substitution::new()),
    }
}

fn dfs(program: &Program, goals: Goal, sbst: Substitution) -> Vec<Substitution> {
    let mut goals = goals.clone();
    match goals.pop() {
        Some(goal) => {
            let mut ans = vec![];
            for clause in program {
                let clause = clause.replace_newvar();
                match unify(&goal, &clause.conclusion) {
                    Some(new_sbst) => {
                        let mut newgoal = goals.clone();
                        let mut goal2 = clause.assumptions.clone();
                        newgoal.append(&mut goal2);
                        dfs(program, newgoal, composite_sbst(&sbst, &new_sbst));
                    }
                    None => (),
                }
            }
            ans
        }
        None => vec![sbst],
    }
}