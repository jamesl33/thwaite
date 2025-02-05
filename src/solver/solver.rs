use std::cmp;
use std::usize;

use crate::cube;
use crate::cube::{Cube, Rotation};
use crate::solver;

/// The pre-compute pattern database for traversing to G1.
static G0: &[u8] = include_bytes!("./group_zero/table.db");

/// The pre-compute pattern database for traversing to G2.
static G1: &[u8] = include_bytes!("./group_one/table.db");

/// The pre-compute pattern database for traversing to G3.
static G2: &[u8] = include_bytes!("./group_two/table.db");

/// The pre-compute pattern database for traversing to G4.
static G3: &[u8] = include_bytes!("./group_three/table.db");

/// Exposes an API to solve the Rubik's Cube using the Thistlewaite-45 method.
#[derive(Debug)]
pub struct Solver {
    /// The cube being solved.
    cube: cube::Cube,
}

impl Solver {
    /// Returns a new solver, which will solve the given cube.
    pub fn new(cube: cube::Cube) -> Solver {
        Solver { cube }
    }

    /// Returns a solution for the target cube, if one can be found.
    pub fn solve(&mut self) -> Option<Vec<cube::Rotation>> {
        // Setup the G0 table
        let g0 = solver::tables::read::<solver::group_zero::Table>(G0);

        // Calculate the rotations to move to G1
        let zero = idas(self.cube, solver::Group::Zero.moves(), &|cube| g0.depth(cube))?;

        // Apply those moves
        self.apply(&zero);

        // Setup the G1 table
        let g1 = solver::tables::read::<solver::group_one::Table>(G1);

        // Calculate the rotations to move to G1
        let one = idas(self.cube, solver::Group::One.moves(), &|cube| g1.depth(cube))?;

        // Apply the moves
        self.apply(&one);

        // Setup the G2 table
        let g2 = solver::tables::read::<solver::group_two::Table>(G2);

        // Calculate the rotations to move to G2
        let two = idas(self.cube, solver::Group::Two.moves(), &|cube| g2.depth(cube))?;

        // Apply the moves
        self.apply(&two);

        // Setup the G3 table
        let g3 = solver::tables::read::<solver::group_three::Table>(G3);

        // Calculate the rotations to move to G3
        let three = idas(self.cube, solver::Group::Three.moves(), &|cube| g3.depth(cube))?;

        // Apply the moves
        //
        // NOTE: This is not strictly required, however, allows us to leave the cube in the solved state.
        self.apply(&three);

        Some(vec![zero, one, two, three].concat())
    }

    /// Applies the given moves to the cube.
    fn apply(&mut self, moves: &[Rotation]) {
        for i in 0..moves.len() {
            self.cube.rotate(moves[i]);
        }
    }
}

/// Perform an iterative deepening A* search, using the given heuristic.
fn idas<F>(cube: Cube, moves: &[Rotation], hueristic: &F) -> Option<Vec<Rotation>>
where
    F: Fn(&Cube) -> usize,
{
    let mut limit = hueristic(&cube);

    loop {
        let (t, path) = dfs(cube, 0, limit, moves, hueristic);

        if path.is_some() {
            return path;
        }

        if t == usize::MAX {
            return None;
        }

        limit = t;
    }
}

/// Perform a depth first search, using the given moves and heuristic returning the minimum cost branch and the moves
/// that have been made to get there.
fn dfs<F>(cube: Cube, g: usize, limit: usize, valid: &[Rotation], hueristic: &F) -> (usize, Option<Vec<Rotation>>)
where
    F: Fn(&Cube) -> usize,
{
    let mut min = usize::MAX;

    for mv in valid {
        if cube.redundant(mv) {
            continue;
        }

        let mut cube = cube;

        cube.rotate(*mv);

        let h = hueristic(&cube);
        let f = g + h;

        if h == 0 {
            return (0, Some(vec![*mv]));
        }

        if f > limit {
            return (f, None);
        }

        let (cost, path) = dfs(cube, g + 1, limit, valid, hueristic);

        if let Some(path) = path {
            return (0, Some(vec![vec![*mv], path].concat()));
        }

        min = cmp::min(min, cost);
    }

    (min, None)
}
