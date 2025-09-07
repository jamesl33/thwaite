use std::cmp;

use serde::{Deserialize, Serialize};

use crate::cube::{Cube, CORNER_ORIENTATIONS, NUM_CORNERS, NUM_EDGES};
use crate::solver::group::Group;
use crate::solver::maths::combinations;

/// The number of combinations of the four (LR slice) edge pieces that we're fixing in this group.
const N_SIZE: usize = combinations(NUM_EDGES, 4);

/// The number of states for the corner orientations, which are being fixed in this group.
const M_SIZE: usize = usize::pow(CORNER_ORIENTATIONS, (NUM_CORNERS - 1) as u32);

/// The size of the G1 pruning table, which is a flat two dimensional array.
const SIZE: usize = N_SIZE * M_SIZE;

/// The size of the lookup table for the edge permutations.
const IDX_LOOKUP_TABLE_SIZE: usize = 2048;

/// Is a lookup table which translates an index in the range 0-2048 into 0-495 allowing us to treat the (LR slice) edge
/// permutations as a binary number.
const IDX_LOOKUP_TABLE: [usize; IDX_LOOKUP_TABLE_SIZE] = idx_lookup_table();

/// The pruning table for the G1.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// The underlying data, where each index represents a cube state and its depth from the solved state.
    data: Vec<usize>,
}

impl Table {
    /// Calculates and returns a new G1 pruning table.
    pub fn new() -> Table {
        g1()
    }

    /// Returns the number of moves the given cube is, from being in G1.
    pub fn depth(&self, cube: &Cube) -> usize {
        self.data[idx(cube)]
    }
}

/// Calculates the edge permutation lookup table, noting that there's 495 variations of eleven digit binary numbers
/// where there's three/four ones.
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

        n += 1;
    }

    table
}

/// Creates a new pattern database for G1.
fn g1() -> Table {
    // As documented the max depth from G0 is ten.
    //
    // http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf
    const DEPTH: usize = 10;

    // We initialize the pruning table at the max depth, then overwrite for cheaper distances
    let mut tab = Table {
        data: vec![DEPTH; SIZE],
    };

    // The zeroth index represents the solved state (e.g. in G1)
    tab.data[0] = 0;

    // Start searching from the solved cube state
    let start: Cube = Cube::new();

    // Perform a depth first search, applying all the valid G1 moves and recording the depth from the solved state
    start.search(Group::One.moves(), DEPTH - 1, &mut |cube, depth| {
        // Calculate the index in the pruning table
        let idx = idx(cube);

        // Only update the pruning table, if we've found a shorter path
        tab.data[idx] = cmp::min(tab.data[idx], depth);
    });

    tab
}

/// Returns the index within the pruning table for the given cube.
fn idx(cube: &Cube) -> usize {
    // Calculate the index for the corner orientations
    let oidx = otoidx(cube.corner_orientations());

    // Calculate the index for the edge permutations
    let pidx = ptoidx(cube.edge_permutations());

    // Calculate the offset in the flat backing array
    oidx * N_SIZE + pidx
}

/// Returns the index within the pruning table for the given corner orientations.
///
/// https://www.jaapsch.net/puzzles/compindx.htm#orient
fn otoidx<const N: usize>(orien: &[usize; N]) -> usize {
    let mut idx: usize = 0;

    for i in 0..N - 1 {
        idx = idx * 3 + orien[i]
    }

    debug_assert!(idx < M_SIZE);

    idx
}

/// Returns the index within the pruning table for the LR slice edges, calculated by treating the permutations as a
/// binary number (ignoring the last edge) then converting that into a number between 0-495 using a lookup table.
fn ptoidx(perms: &[usize; NUM_EDGES]) -> usize {
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        if perms[i] <= 7 {
            continue;
        }

        dec += 2_usize.pow(10 - i as u32);
    }

    let idx = IDX_LOOKUP_TABLE[dec];

    debug_assert!(idx < N_SIZE);

    idx
}
