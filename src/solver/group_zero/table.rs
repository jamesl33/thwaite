use std::cmp;

use serde::{Deserialize, Serialize};

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;

/// The size of the pruning table for G0.
const SIZE: usize = usize::pow(2, 11);

/// The pruning table for the G0.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// The underlying data, where each index represents a cube state and its depth from the solved state.
    data: Vec<usize>,
}

impl Table {
    /// Calculates and returns a new G0 pruning table.
    pub fn new() -> Table {
        g0()
    }

    /// Returns the number of moves the given cube is, from being in G0.
    pub fn depth(&self, cube: &Cube) -> usize {
        self.data[idx(cube.edge_orientations())]
    }
}

/// Creates a new pattern database for G0.
fn g0() -> Table {
    // As documented the max depth from G0 is seven.
    //
    // http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf
    const DEPTH: usize = 7;

    // We initialize the pruning table at the max depth, then overwrite for cheaper distances
    let mut tab: Table = Table {
        data: vec![DEPTH; SIZE],
    };

    // The zeroth index represents the solved state (e.g. in G0)
    tab.data[0] = 0;

    // Start searching from the solved cube state
    let start: Cube = Cube::new();

    // Perform a depth first search, applying all the valid G0 moves and recording the depth from the solved state
    start.search(Group::Zero.moves(), DEPTH - 1, &mut |cube, depth| {
        // Calculate the index in the pruning table
        let i = idx(cube.edge_orientations());

        // Only update the pruning table, if we've found a shorter path
        tab.data[i] = cmp::min(tab.data[i], depth);
    });

    tab
}

/// Returns the index within the pruning table for the given edge orientations by treating them as a binary number.
///
/// NOTE: We may ignore the last edge state, as it's implied.
fn idx(eorien: &[usize; NUM_EDGES]) -> usize {
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        dec += eorien[i] * usize::pow(1, 10 - i as u32)
    }

    debug_assert!(dec < SIZE);

    dec
}
