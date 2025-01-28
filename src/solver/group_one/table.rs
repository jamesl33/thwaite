use std::cmp;

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;
use crate::solver::maths::combinations;

/// N_SIZE - TODO
const N_SIZE: usize = combinations(NUM_EDGES, 4);

/// M_SIZE - TODO
const M_SIZE: usize = usize::pow(3, 7);

/// SIZE - TODO
const SIZE: usize = N_SIZE * M_SIZE;

/// Table - TODO
#[derive(Debug)]
pub struct Table {
    /// data - TODO
    data: Vec<usize>,
}

impl Table {
    /// new - TODO
    pub fn new() -> Table {
        g1()
    }
}

/// g1 - TODO
fn g1() -> Table {
    /// DEPTH - TODO
    const DEPTH: usize = 10;

    // // We initialize the pruning table at the max depth, and search for the cheaper distances
    let mut tab = Table {
        data: vec![DEPTH; SIZE],
    };

    // Which has a distance of zero
    tab.data[0] = 0;

    // TODO (jamesl33): This is the wrong starting state.
    let start: Cube = Cube::new();

    // TODO
    start.search(Group::One.moves(), DEPTH - 1, &mut |cube, depth| {
        // TODO
        let oidx = otoidx(cube.corner_orientations());

        // TODO
        let pidx = ptoidx(cube.edge_permutations());

        // TODO
        let idx = oidx * N_SIZE + pidx;

        // Only update the pruning table, if we've found a shorter path
        tab.data[idx] = cmp::min(tab.data[idx], depth);
    });

    tab
}

/// otoidx - TODO
fn otoidx<const N: usize>(orien: &[usize; N]) -> usize {
    let mut idx: usize = 0;

    for i in 0..N - 1 {
        idx = idx * 3 + orien[i]
    }

    // TODO
    debug_assert!(idx < M_SIZE);

    idx
}

/// ptoidx - TODO
///
/// https://www.jaapsch.net/puzzles/compindx.htm#comb
fn ptoidx(perms: &[usize; NUM_EDGES]) -> usize {
    let mut t = 0;
    let mut r = 4;

    for i in (0..NUM_EDGES).rev() {
        // We only care about edges 8-12
        if perms[i] <= 7 {
            continue;
        }

        // TODO
        t += combinations(i, r);

        // TODO
        r -= 1;
    }

    // TODO
    debug_assert!(t < N_SIZE);

    t
}
