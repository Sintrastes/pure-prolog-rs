
use crate::unifiable::*;
use std::hash::Hash;
use std::collections::HashMap;
use derive_more::{Display};

#[derive(PartialEq, Copy, Clone, Display, Debug)]
pub enum Term<'a, T: 'a, V: 'a> {
    IntLiteral(i32),
    DoubleLiteral(f32),
    StringLiteral(&'a String),
    Identifier(T),
    Var(V),
    #[display(fmt = "")]
    Comp(&'a Term<'a, T,V>, &'a Vec<Term<'a, T,V>>)
}

impl <'a,T: Clone + Eq, V: Clone + Eq + Hash + Copy> Unifiable<'a,Term<'a,T,V>,V> for Term<'a,T,V> {
    fn variables(term: &Term<T,V>) -> Vec<V> {
        match term {
            Term::Var(x) => {
                return vec!(*x);
            }
            _ => { return vec!(); }
        }
    }
    fn subs(unifier: &'a Unifier<'a,V,Term<'a,T,V>>, term: &'a Term<T,V>) -> &'a Term<'a,T,V> {
        match term {
            Term::Var(x) => {
                match unifier.0.get(x) {
                    None => { return term; }
                    Some(y) => { return y; }
                }
            }
            Term::Comp(t,ts) => {
                let term = Box::new(
                    Term::Comp(
                        Self::subs(unifier, t), ts
                    )
                );
                return Box::leak(term);
            }
            x => { return x; }
        }
    }
    fn unify(first_term: &Term<'a,T,V>, second_term: &Term<'a,T,V>) -> Option<&'a Unifier<'a,V,Term<'a,T,V>>> {
        match (first_term, second_term) {
            (Term::Comp(x,xs), Term::Comp(y,ys)) => {
                let u = Self::unify(*x,*y);
                let us = Self::unify_all(*xs,*ys);
                match (u,us) {
                    (Some(u_), Some(us_)) => { 
                        let u_ref = Box::leak(Box::new(u_));
                        let us_ref = Box::leak(Box::new(us_));
                        return Some(Self::compose(u_ref, us_ref)); 
                    }
                    _ => { return None; }
                }
            }
            (Term::Var(x), y) => {
                let mut hm: HashMap<V,&'a Term<T,V>> = HashMap::new();
                let y_term = Box::leak(Box::new(y.clone()));
                hm.insert(*x, y_term);
                let return_hm = Box::leak(Box::new(Unifier(hm)));
                return Some(return_hm);
            }
            (y, Term::Var(x)) => {
                let mut hm: HashMap<V,&'a Term<T,V>> = HashMap::new();
                let y_term = Box::leak(Box::new(y.clone()));
                hm.insert(*x, y_term);
                let return_hm = Box::leak(Box::new(Unifier(hm)));
                return Some(return_hm);
            }
            (Term::Var(x), Term::Var(y)) => {
                let mut hm: HashMap<V,&'a Term<T,V>> = HashMap::new();
                let y_term = Box::leak(Box::new(Term::Var(*y)));
                hm.insert(*x, y_term);
                let return_hm = Box::leak(Box::new(Unifier(hm)));
                return Some(return_hm);
            }
            (x,y) => {
                if x == y {
                    return Some(Box::leak(Box::new(Unifier(HashMap::new()))));
                } else {
                    return None;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Clause<'a,T,V> {
    pub head: Term<'a,T,V>,
    pub body: Vec<Term<'a,T,V>>
}

#[derive(Clone, Display, Debug)]
pub struct Program<'a,T,V> {
    pub clauses: Vec<Clause<'a,T,V>>
}

#[derive(Clone, Display, Debug)]
pub struct Goal<'a,T,V> {
    pub terms: Vec<Term<'a,T,V>>
}