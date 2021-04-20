
use std::iter::*;
use std::hash::Hash;
use crate::unifiable::*;
use crate::term::*;
use crate::tree::*;

fn freshen<'a,T,V>(vars: Vec<V>, clause: &'a Clause<'a, T,V>) -> &'a Clause<'a,T,V> {
    return clause;
}

fn nontriv<T,V: Eq>(x: V, term: &Term<T,V>) -> bool {
    match term {
        Term::Var(y) if x == *y => {
            false
        }
        _ => { true }
    }
}

pub fn make_search_tree<'a, T: Clone + Eq, V: Eq + Hash + Copy>(
    goal: &'static Goal<'a,T,V>, program: &'a Program<'a, T,V>, assignments: Unifier<'a, V, Term<T,V>>
) -> &'a Tree<Goal<'a,T,V>,Unifier<'a, V, Term<'a, T,V>>> {
    if goal.terms.len() > 0 {
        use mdo::iter::{bind};
        let varsInGoal: Vec<V> = goal.terms.iter()
            .flat_map(|x| Term::<T,V>::variables(x)).collect();

        let new_goal = Box::leak(Box::new(goal));
        let t = &(*new_goal).terms[0];
        let ts = &new_goal.terms[1..];
        
        let program_copy = program.clauses.clone();
        let trees = mdo! {
            clause =<< program.clauses.iter();
            let Clause {head: clauseHead, body: clauseBody} = freshen(varsInGoal.clone(), clause);
            ret match Term::unify(&clauseHead, &t) {
                Some(unifier) => {
                    let new_clause_body = Box::leak(Box::new(clauseBody.clone()));
                    let new_goal: &'a Goal<'a,T,V> = Box::leak(Box::new(Goal { 
                        terms: new_clause_body.iter().chain(ts.iter())
                            .map(|x| Term::subs(unifier,x).clone()).collect()
                    }));
                    let mut new_assignments = assignments.0.clone();
                    new_assignments.extend(unifier.0.clone());
                    let new_tree = make_search_tree(&new_goal, program, Unifier(new_assignments));
                    Box::leak(Box::new(vec!(new_tree)))
                }
                None => { 
                    Box::leak(Box::new(vec!()))
                }
            }
        };
        let lazy_trees = Box::new(trees);
        return Box::leak(Box::new(
            Tree::Branch(
                goal.clone(), lazy_trees
            )
        ))
    } else {
      return Box::leak(Box::new(
        Tree::Node(
          Unifier(assignments.0.iter()
            .filter(|(x,&y)| nontriv(**x,y))
            .map(|(&x,&y)| (x,y))
            .collect())
        )
      ));
    }
}