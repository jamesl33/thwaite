use serde::{Deserialize, Serialize};

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::generate::bfs;
use crate::solver::group::Group;
use crate::solver::maths::ptoidx;

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

    // The edge orientation coordinate alone isn't closed under the move action - its evolution depends on the
    // full edge permutation too (see `crate::cube::Cube::rotate_up`) - so the search key pairs it with the edge
    // permutation rank to avoid silently missing states. See `crate::solver::generate::bfs_from` for the full
    // explanation.
    Table {
        data: bfs(
            Group::Zero.moves(),
            DEPTH,
            SIZE,
            |cube| idx(cube.edge_orientations()),
            |cube| (idx(cube.edge_orientations()), ptoidx(cube.edge_permutations())),
        ),
    }
}

/// Returns the index within the pruning table for the given edge orientations by treating them as a binary number.
///
/// NOTE: We may ignore the last edge state, as it's implied.
fn idx(eorien: &[usize; NUM_EDGES]) -> usize {
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        dec += eorien[i] * usize::pow(2, 10 - i as u32)
    }

    debug_assert!(dec < SIZE);

    dec
}
