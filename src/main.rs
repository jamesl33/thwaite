#![allow(dead_code)]

#[macro_use]
extern crate arrayref;

mod cube;
use cube::*;

mod solver;
use solver::*;

fn main() {
    let c: Cube = Cube::new();
    let s: Solver = Solver::new(c);
    println!("{:?}", s.solve());
}
