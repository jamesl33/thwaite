use std::cmp;
use std::io::Read;
use std::usize;

use bytes::Buf;

use crate::cube;
use crate::cube::{Cube, Rotation};
use crate::solver;

/// G0 - TODO
static G0: &[u8] = include_bytes!("./group_zero/table.db");

/// G1 - TODO
static G1: &[u8] = include_bytes!("./group_one/table.db");

/// G2 - TODO
static G2: &[u8] = include_bytes!("./group_two/table.db");

/// G3 - TODO
static G3: &[u8] = include_bytes!("./group_three/table.db");

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
        let g0 = read_table::<solver::group_zero::Table>(G0);

        // TODO
        let zero = idas(self.cube, solver::Group::Zero.moves(), &|cube| g0.depth(cube))?;

        self.apply(&zero);

        // TODO
        let g1 = read_table::<solver::group_one::Table>(G1);

        // TODO
        let one = idas(self.cube, solver::Group::One.moves(), &|cube| g1.depth(cube))?;

        self.apply(&one);

        // TODO
        let g2 = read_table::<solver::group_two::Table>(G2);

        // TODO
        let two = idas(self.cube, solver::Group::Two.moves(), &|cube| g2.depth(cube))?;

        self.apply(&two);

        // TODO
        let g3 = read_table::<solver::group_three::Table>(G3);

        // TODO
        let three = idas(self.cube, solver::Group::Three.moves(), &|cube| g3.depth(cube))?;

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

/// read_table - TODO
///
/// TODO (jamesl33): Add a binary to generate these, and include them in the solver binary.
fn read_table<'a, T: Sized>(table: &[u8]) -> T
where
    T: serde::de::DeserializeOwned,
{
    // TODO
    let mut decoder = snap::read::FrameDecoder::new(table.reader());

    // TODO
    let mut encoded = vec![];

    // TODO
    decoder.read_to_end(&mut encoded).unwrap();

    // TODO
    let decoded = bincode::deserialize(&encoded).unwrap();

    decoded
}
