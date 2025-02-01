use std::cmp;
use std::os::unix::thread::JoinHandleExt;
use std::usize;

use crate::cube;
use crate::cube::{Cube, Rotation};
use crate::solver;
use crate::solver::group::Group;

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
    pub fn solve(&mut self) -> Option<Vec<cube::Rotation>> {
        let g0 = solver::group_zero::Table::new();

        // TODO
        let zero = idas(
            self.cube,
            solver::Group::Zero.moves(),
            &| cube | g0.depth(cube)
        )?;

        self.apply(&zero);

        // TODO
        let g1 = solver::group_one::Table::new();

        // TODO
        let one = idas(
            self.cube,
            solver::Group::One.moves(),
            &| cube | g1.depth(cube)
        )?;

        self.apply(&one);

        // TODO
        let g2 = solver::group_two::Table::new();

        // TODO
        let two = idas(
            self.cube,
            solver::Group::Two.moves(),
            &| cube | g2.depth(cube)
        )?;

        self.apply(&two);

        // TODO
        let g3 = solver::group_three::Table::new();

        // TODO
        let three = idas(
            self.cube,
            solver::Group::Three.moves(),
            &| cube | g3.depth(cube)
        )?;

        self.apply(&three);

        Some(vec![zero, one, two, three].concat())
    }

    /// apply - TODO
    fn apply(&mut self, moves: &[Rotation]) {
        for i in 0..moves.len() {
            self.cube.rotate(moves[i]);
        }
    }
}

/// idas - TODO
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

/// dfs - TODO
fn dfs<F>(
    cube: Cube,
    g: usize,
    limit: usize,
    valid: &[Rotation],
    hueristic: &F
) -> (usize, Option<Vec<Rotation>>)
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
            return (0, Some(vec![vec![*mv], path].concat()))
        }

        min = cmp::min(min, cost);
    }

    (min, None)
}
