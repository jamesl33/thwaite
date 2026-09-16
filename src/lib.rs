#![allow(dead_code)]

#[macro_use]
extern crate arrayref;

pub mod cube;
pub use cube::*;

pub mod solver;
pub use solver::*;
