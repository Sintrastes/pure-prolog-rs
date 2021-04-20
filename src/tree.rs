
use std::iter::*;
use derive_more::{Display};

#[derive(Display)]
pub enum Tree<B, A> {
    Node(A),
    #[display(fmt = "Branch({})", _0)]
    Branch(B, Box<dyn Iterator<Item = Tree<B,A>>>),
}

#[derive(Clone)]
pub enum MyIterator<A> {
    Done,
    Continue(Chain<Once<A>, Box<MyIterator<A>>>)
}

impl <A> Iterator for MyIterator<A> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        match self {
            MyIterator::Done => {
                return None;
            }
            MyIterator::Continue(iter) => {
                return iter.next()
            }
        }
    }
}

fn asStreamIter<A>(mut iter: impl Iterator<Item = A>) -> MyIterator<A> {
    return match iter.next() {
        None => { MyIterator::Done }
        Some(x) => {
            MyIterator::Continue(once(x).chain(Box::new(asStreamIter(iter))))
        }
    }
}

/*
fn trav<A: Clone,B: Clone>(trees: MyIterator<Tree<B,A>>) -> MyIterator<A> {
    let newTrees = trees.clone();
    match trees {
      MyIterator::Done => { return MyIterator::Done; }
      MyIterator::Continue(mut iter) => {
        let head = iter.next();
        match head {
            Some(Tree::Node(x)) => {
                return MyIterator::Continue(once(x).chain(Box::new(trav(newTrees))));
            }
            Some(Tree::Branch(_, st)) => {
                // TODO: Need to implement this.
                let test: MyIterator<Tree<B, A>> = MyIterator::Done; // asStreamIter(iter.clone().chain(st.iter().map(|x| x.clone())));
                return trav(test);
            }
            None => { return MyIterator::Done; }
        }
    }
  }
}

/** Implementation of breadth-first search for trees. */
pub fn bfs<A: 'static + Clone, B: 'static + Clone>(tree: &Tree<B,A>) -> MyIterator<A> {
    let init = MyIterator::Continue(once(tree.clone()).chain(Box::new(MyIterator::Done)));
    let result = trav(init);
    return result;
}
*/

/** Imperative implementation of depth-first search for trees. */
pub fn dfs<A: Clone, B: Clone>(tree: &mut Tree<B,A>, visit: &mut dyn FnMut(&A) -> ()) {
    match tree {
        Tree::Node(x) => { visit(x) }
        Tree::Branch(_, xs) => {
            for mut x in xs {
                dfs(&mut x, visit);
            }
        }
    }
}

pub fn dfs_results<A: Clone, B: Clone>(tree: &mut Tree<B,A>) -> Vec<A> {
    let mut results = vec!();
    dfs(tree, &mut |x| results.push(x.clone()));
    return results;
}