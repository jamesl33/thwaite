use std::cmp;

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;

/// SIZE - TODO
const SIZE: usize = 2048;

/// Table - TODO
#[derive(Debug)]
pub struct Table {
    /// data - TODO
    data: [usize; SIZE],
}

impl Table {
    /// new - TODO
    pub fn new() -> Table {
        g0()
    }
}

/// g0 - TODO
fn g0() -> Table {
    /// DEPTH - TODO
    const DEPTH: usize = 7;

    // We initialize the pruning table at the max depth, and search for the cheaper distances
    let mut tab: Table = Table { data: [DEPTH; SIZE] };

    // Which has a distance of zero
    tab.data[0] = 0;

    // We start searching from a solved cube
    let start: Cube = Cube::new();

    // Perform a depth first search by applying turns to the cube, and recording the depth from the solved state, in
    // the pruning table.
    start.search(Group::Zero.moves(), DEPTH - 1, &mut |cube, depth| {
        // Calculate the index in the pruning table
        let i = idx(cube.edge_orientations());

        // Only update the pruning table, if we've found a shorter path
        tab.data[i] = cmp::min(tab.data[i], depth);
    });

    tab
}

/// idx - TODO
fn idx(eorien: &[isize; NUM_EDGES]) -> usize {
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        dec += eorien[i] as usize * (2 as usize).pow(10 - i as u32)
    }

    dec
}
