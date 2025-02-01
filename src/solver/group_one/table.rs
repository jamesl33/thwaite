use std::cmp;

use serde::{Deserialize, Serialize};

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;
use crate::solver::maths::combinations;

/// N_SIZE - TODO
const N_SIZE: usize = combinations(NUM_EDGES, 4);

/// M_SIZE - TODO
const M_SIZE: usize = usize::pow(3, 7);

/// SIZE - TODO
const SIZE: usize = N_SIZE * M_SIZE;

/// IDX_LOOKUP_TABLE_SIZE - TODO
const IDX_LOOKUP_TABLE_SIZE: usize = 2048;

/// IDX_LOOKUP_TABLE - TODO
///
/// TODO (jamesl33): Converts a range from 2048, to 495.
const IDX_LOOKUP_TABLE: [usize; IDX_LOOKUP_TABLE_SIZE] = idx_lookup_table();

/// idx_lookup - TODO
const fn idx_lookup_table() -> [usize; IDX_LOOKUP_TABLE_SIZE] {
    let mut n: usize = 0;
    let mut idx = 0;
    let mut table = [0; IDX_LOOKUP_TABLE_SIZE];

    while n < 2048 {
        let c = n.count_ones();

        if c == 3 || c == 4 {
            table[n] = idx;
            idx += 1;
        }

        n = n + 1;
    }

    table
}

/// Table - TODO
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// data - TODO
    data: Vec<usize>,
}

impl Table {
    /// new - TODO
    pub fn new() -> Table {
        g1()
    }

    /// depth - TODO
    pub fn depth(&self, cube: &Cube) -> usize {
        self.data[idx(cube)]
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
        let idx = idx(cube);

        // Only update the pruning table, if we've found a shorter path
        tab.data[idx] = cmp::min(tab.data[idx], depth);
    });

    tab
}

/// idx - TODO
fn idx(cube: &Cube) -> usize {
    // TODO
    let oidx = otoidx(cube.corner_orientations());

    // TODO
    let pidx = ptoidx(cube.edge_permutations());

    // TODO
    oidx * N_SIZE + pidx
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
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        if perms[i] <= 7 {
            continue;
        }

        dec += (2 as usize).pow(10 - i as u32);
    }

    let idx = IDX_LOOKUP_TABLE[dec];

    idx
}
