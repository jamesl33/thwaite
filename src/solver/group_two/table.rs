use std::cmp;

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;

/// N_SIZE - TODO
const N_SIZE: usize = 495;

/// M_SIZE - TODO
const M_SIZE: usize = 2187;

/// IDX_LOOKUP_TABLE_SIZE - TODO
const IDX_LOOKUP_TABLE_SIZE: usize = 2048;

/// IDX_LOOKUP_TABLE - TODO
///
/// TODO (jamesl33): Converts a range from 2048, to 495.
const IDX_LOOKUP_TABLE: [usize; IDX_LOOKUP_TABLE_SIZE] = idx_lookup_table();

/// Table - TODO
#[derive(Debug)]
pub struct Table {
    /// data - TODO
    data: Vec<[usize; M_SIZE]>,
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
        data: vec![[DEPTH; M_SIZE]; N_SIZE],
    };

    // Which has a distance of zero
    tab.data[0][0] = 0;

    // TODO (jamesl33): This is the wrong starting state.
    let start: Cube = Cube::new();

    // TODO
    start.search(Group::One.moves(), DEPTH - 1, &mut |cube, depth| {
        let oidx = otoidx(cube.corner_orientations());
        let pidx = ptoidx(cube.edge_permutations());

        // Only update the pruning table, if we've found a shorter path
        tab.data[pidx][oidx] = cmp::min(tab.data[pidx][oidx], depth);
    });

    tab
}

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

/// otoidx - TODO
fn otoidx<const N: usize>(orien: &[isize; N]) -> usize {
    let mut idx: usize = 0;

    for i in 0..N - 1 {
        idx = idx * 3 + orien[i] as usize;
    }

    // TODO
    debug_assert!(idx < M_SIZE);

    idx
}

/// ptoidx - TODO
fn ptoidx(perms: &[isize; NUM_EDGES]) -> usize {
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        // We only care about the corners 9-12
        if perms[i] <= 7 {
            continue;
        }

        dec += (2 as usize).pow(10 - i as u32)
    }

    let idx = IDX_LOOKUP_TABLE[dec];

    // TODO
    debug_assert!(idx < N_SIZE);

    idx
}
