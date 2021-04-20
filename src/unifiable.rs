
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Unifier<'a,V,T>(pub HashMap<V, &'a T>);

impl <'a,V,T> Display for Unifier<'a,V,T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

pub trait Compose<V,T> {
  fn compose(&self, other: Unifier<V,T>) -> Unifier<V,T>;
}

pub trait Unifiable<'a,T: Clone,V: Eq + Clone + Copy + Hash> {
  fn variables(term: &T) -> Vec<V>;
  fn subs(unifier: &'a Unifier<'a, V,T>, term: &'a T) -> &'a T;
  fn unify(first_term: &T, second_term: &T) -> Option<&'a Unifier<'a, V,T>>;
  fn unify_all(first_terms: &'a Vec<T>, second_terms: &'a Vec<T>) -> Option<&'a Unifier<'a, V,T>> {
    match (&first_terms[..], &second_terms[..]) {
      (&[],&[]) => { 
        Some(Box::leak(Box::new(Unifier(HashMap::new()))))
      }
      _ => {
        if !(*first_terms).is_empty() && !(*second_terms).is_empty() {
          let ts = first_terms[1..].iter();
          let rs = second_terms[1..].iter();
          let r = &second_terms[0];
          let t = &first_terms[0];

          use mdo::option::{bind};
          mdo! {
              u1 =<< Self::unify(&t,&r);
              let _ts = Box::leak(Box::new(
                ts.map(|x| 
                  Self::subs(u1,x).clone()).collect()
                ));
              let _rs = Box::leak(Box::new(
                rs.map(|x| 
                  Self::subs(u1,x).clone()).collect()
                ));
              u2 =<< Self::unify_all(_ts, _rs);
              ret Some(Self::compose(u1,u2))
          }
        } else {
            None
        } 
      }
    }
  }
  fn compose(
    this: &'a Unifier<'a, V,T>,
    other: &'a Unifier<'a, V,T>
  ) -> &'a Unifier<'a, V,T> {
  
      let Unifier(this_hm) = this;
      let Unifier(other_hm) = other;
      
      let mut unifier: HashMap<V, &'a T> = this_hm
        .iter().map(|(&x,&y)| {
          (x, Self::subs(other, y))
      }).collect();
  
      unifier.extend(other_hm.clone());
      return Box::leak(Box::new(Unifier(unifier)));
  }
}