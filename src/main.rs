#![allow(dead_code)]
mod cube;
use cube::*;

mod solver;
use solver::*;

fn main() {
    let c: Cube = Cube::new();
    let s: Solver = Solver::new(c);
    println!("{:?}", s.solve());
}
