
#![feature(once_cell)]
#![feature(fn_box)]
#![feature(untagged_unions)]
#![feature(unsized_locals)]
#[macro_use] extern crate mdo;

mod tree;
mod unifiable;
mod term;
mod interp;
use std::collections::HashMap;

fn main() {
    use unifiable::*;

    let my_tree: tree::Tree<i32, i32> = tree::Tree::Node(42);

    let x: term::Term<&str,&str> = term::Term::Identifier("x");
    let y: term::Term<&str,&str> = term::Term::Identifier("y");
    let z: term::Term<&str,&str> = term::Term::Identifier("z");
    let p: &term::Term<&str,&str> = Box::leak(Box::new(term::Term::Identifier("p")));

    let X: term::Term<&str,&str> = term::Term::Var("X");
    let Y: term::Term<&str,&str> = term::Term::Var("Y");
    let Z: term::Term<&str,&str> = term::Term::Var("Z");

    let x_vec = Box::leak(Box::new(vec!(x)));
    let y_vec = Box::leak(Box::new(vec!(y)));
    let X_vec = Box::leak(Box::new(vec!(X)));
    let XY_vec = Box::leak(Box::new(vec!(X,Y)));
    let YZ_vec = Box::leak(Box::new(vec!(Y,Z)));
    let XZ_vec = Box::leak(Box::new(vec!(X,Z)));
    let xy_vec = Box::leak(Box::new(vec!(x,y)));
    let yz_vec = Box::leak(Box::new(vec!(y,z)));
    let term_1 = term::Term::Comp(&p, &x_vec);
    let term_2 = term::Term::Comp(&p, &X_vec);

    let unified = term::Term::unify(&term_1, &term_2);

    println!("----- UNIFIER TEST: ----- ");

    println!("  Size of unifier is: {}", unified.unwrap().0.len());

    for x in unified.unwrap().0.iter() {
        println!("  {{ {} = {} }}", x.0, x.1);
    }

    println!("----- SEARCH TREE TEST: ----- ");
    {

    let clause_1 = term::Clause { 
        head: term::Term::Comp(&p, x_vec),
        body: vec!() 
    };
    let clause_2 = term::Clause { 
        head: term::Term::Comp(&p, y_vec),
        body: vec!() 
    };
    let program = Box::leak(Box::new(term::Program {
        clauses: vec!(clause_1, clause_2)
    }));

    let goal = Box::leak(Box::new(term::Goal { 
        terms: vec!(term::Term::Comp(&p, X_vec))
    }));

    let mut tree = interp::make_search_tree(goal, program, Unifier(HashMap::new()));
    let solutions = tree::dfs_results(&mut tree);

    // println!("{:?}", tree);

    for solution in solutions {
        for binding in solution.0 {
            println!("{} = {}", binding.0, binding.1);
        }
    }

    }
    println!("----- SEARCH TREE TEST 2 (INFERENCE): ----- ");
    {

    let p_trans = term::Clause { 
        head: term::Term::Comp(&p, XZ_vec),
        body: vec!(
            term::Term::Comp(&p, XY_vec),
            term::Term::Comp(&p, YZ_vec)
        )
    };

    let fact_1 = term::Clause { 
        head: term::Term::Comp(&p, xy_vec),
        body: vec!()
    };

    let fact_2 = term::Clause { 
        head: term::Term::Comp(&p, yz_vec),
        body: vec!()
    };

    let goal = Box::leak(Box::new(term::Goal { 
        terms: vec!(term::Term::Comp(&p, YZ_vec))
    }));

    let program = Box::leak(Box::new(term::Program {
        clauses: vec!(p_trans, fact_1, fact_2)
    }));

    let tree = interp::make_search_tree(goal, program, Unifier(HashMap::new()));
    // println!("{:?}", tree);
    // let solutions = tree::dfs_results(tree);

    /*
    for solution in solutions {
        for binding in solution.0 {
            println!("{} = {}", binding.0, binding.1);
        }
    }
    */    

    }
}
