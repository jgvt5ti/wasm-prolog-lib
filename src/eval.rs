use crate::syntax::*;
use std::iter::zip;

pub fn execute(program: &Program, goal: &Goal) -> Answer {
    let sbst = Substitution::new();
    let goal_vars = goals_free_vars(goal);
    let sbsts = dfs(&program, &goal, &sbst);
    if goal_vars.is_empty() {
        Answer::Valid(sbsts.is_some())
    } else {
        match sbsts {
            Some(sbsts) => {
                let ans = sbsts
                    .iter()
                    .map(|sbst| select_vars(sbst, &goal_vars))
                    .collect();
                Answer::Satisfiable(Some(ans))
            }
            None => Answer::Satisfiable(None),
        }
    }
}

fn unify(t1: &Term, t2: &Term) -> Option<Substitution> {
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

pub fn dfs(program: &Program, goals: &Goal, sbst_root: &Substitution) -> Option<Vec<Substitution>> {
    let mut goals = goals.clone();
    match goals.pop() {
        Some(goal) => {
            let mut ans = vec![];
            for clause in program {
                let clause = clause.replace_newvar();
                match unify(&goal, &clause.conclusion) {
                    Some(sbst_now) => {
                        let new_sbst = composite_sbst(&sbst_root, &sbst_now);
                        let mut newgoal = goals.clone();
                        let raw_goal2 = clause.assumptions.clone();
                        let mut goal2 = goal_substitute(&raw_goal2, &sbst_now);
                        newgoal.append(&mut goal2);
                        match dfs(program, &newgoal, &new_sbst) {
                            Some(sbst_leefs) => {
                                let mut local_ans = sbst_leefs
                                    .iter()
                                    .map(|sb| composite_sbst(&new_sbst, &sb))
                                    .collect::<Vec<Substitution>>();
                                ans.append(&mut local_ans);
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
            }
            if ans.is_empty() {
                None
            } else {
                Some(ans)
            }
        }
        None => Some(vec![sbst_root.clone()]),
    }
}
