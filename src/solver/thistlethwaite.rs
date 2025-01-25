use crate::cube;
use crate::solver;

/// Solver - TODO
#[derive(Debug)]
pub struct Solver {
    /// cube - TODO
    cube: cube::Cube,
}

impl Solver {
    /// new - TODO
    pub fn new(cube: cube::Cube) -> Solver {
        Solver { cube }
    }

    /// solve - TODO
    ///
    /// TODO (jamesl33): Add an algorithm type to abstract a number of rotations (and perform basic reduction).
    pub fn solve(&self) -> Vec<cube::Rotation> {
        vec![]
    }
}
