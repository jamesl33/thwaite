use serde::{Deserialize, Serialize};

use crate::cube::{Cube, NUM_CORNERS, NUM_EDGES};
use crate::solver::generate::bfs;
use crate::solver::kociemba::phase::PHASE_TWO_VALID_MOVES;
use crate::solver::maths::{factorial, ptoidx};

/// The number of corner permutations, which are being fixed in phase two.
const CORNER_PERM_STATES: usize = factorial(NUM_CORNERS);

/// The number of permutations of the eight non LR-slice edges, which are being fixed in phase two.
const NONSLICE_EDGE_PERM_STATES: usize = factorial(8);

/// The number of permutations of the four LR-slice edges, which are being fixed in phase two.
const SLICE_EDGE_PERM_STATES: usize = factorial(4);

/// The size of the corner-permutation pruning table, a flat two dimensional array.
const SIZE_CORNER: usize = CORNER_PERM_STATES * SLICE_EDGE_PERM_STATES;

/// The size of the edge-permutation pruning table, a flat two dimensional array.
const SIZE_EDGE: usize = NONSLICE_EDGE_PERM_STATES * SLICE_EDGE_PERM_STATES;

/// As documented by Kociemba, phase two requires at most eighteen moves.
///
/// https://kociemba.org/cube.htm
const DEPTH: usize = 18;

/// The pruning table for phase two; the depth to solve the cube (having already reached phase one's target
/// group) is the maximum of the two coordinates, since together they'd be too large to store densely.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// Depths keyed by corner permutation and LR-slice edge permutation.
    corner: Vec<usize>,

    /// Depths keyed by non LR-slice edge permutation and LR-slice edge permutation.
    edge: Vec<usize>,
}

impl Table {
    /// Calculates and returns a new phase two pruning table.
    pub fn new() -> Table {
        phase_two()
    }

    /// Returns the number of moves the given cube is, from being solved.
    pub fn depth(&self, cube: &Cube) -> usize {
        std::cmp::max(self.corner[corner_idx(cube)], self.edge[edge_idx(cube)])
    }
}

/// Creates a new pattern database for phase two.
fn phase_two() -> Table {
    Table {
        corner: bfs(&PHASE_TWO_VALID_MOVES, DEPTH, SIZE_CORNER, corner_idx, corner_idx),
        edge: bfs(&PHASE_TWO_VALID_MOVES, DEPTH, SIZE_EDGE, edge_idx, edge_idx),
    }
}

/// Returns the index within the corner-permutation pruning table for the given cube.
fn corner_idx(cube: &Cube) -> usize {
    ptoidx(cube.corner_permutations()) * SLICE_EDGE_PERM_STATES + ptoidx(&slice_edges(cube.edge_permutations()))
}

/// Returns the index within the edge-permutation pruning table for the given cube.
fn edge_idx(cube: &Cube) -> usize {
    ptoidx(&nonslice_edges(cube.edge_permutations())) * SLICE_EDGE_PERM_STATES
        + ptoidx(&slice_edges(cube.edge_permutations()))
}

/// Extracts the four LR-slice edges (piece ids 8-11, guaranteed by phase one to occupy slots 8-11) and
/// relabels them to rank 0-3, for use with `ptoidx`.
fn slice_edges(perms: &[usize; NUM_EDGES]) -> [usize; 4] {
    std::array::from_fn(|i| perms[8 + i] - 8)
}

/// Extracts the eight non LR-slice edges (slots 0-7, already ranked 0-7 since piece ids 0-7 are contiguous),
/// for use with `ptoidx`.
fn nonslice_edges(perms: &[usize; NUM_EDGES]) -> [usize; 8] {
    std::array::from_fn(|i| perms[i])
}
