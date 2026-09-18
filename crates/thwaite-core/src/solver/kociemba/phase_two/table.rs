use serde::{Deserialize, Serialize};

use crate::cube::{Cube, Symmetry, NUM_CORNERS, NUM_EDGES, SYMMETRIES};
use crate::solver::search::bfs;
use crate::solver::kociemba::phase::PHASE_TWO_VALID_MOVES;
use crate::solver::maths::{factorial, idxtoperm, ptoidx};

use super::symmetry::{CORNER_SYM, NUM_CORNER_CLASSES, REP_CORNER};

/// The number of corner permutations, which are being fixed in phase two.
pub(super) const CORNER_PERM_STATES: usize = factorial(NUM_CORNERS);

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
/// group) is the maximum of three coordinates. `corner` and `edge` are only weakly correlated, so
/// `corner_edge_sym` adds a genuinely joint corner/edge coordinate (symmetry-reduced via `SYMMETRIES` - see
/// `super::symmetry` - to stay tractable to store).
///
/// All three coordinates are needed: `corner_edge_sym` drops LR-slice-edge permutation entirely, so `corner`/
/// `edge` remain the only signal once that's the dominant remaining work. Empirically, each of `corner`/`edge`
/// alone exceeds `corner_edge_sym` on ~38% of sampled reachable states (by several moves) - dropping either
/// would meaningfully weaken the heuristic.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// Depths keyed by corner permutation and LR-slice edge permutation.
    #[serde(with = "serde_bytes")]
    corner: Vec<u8>,

    /// Depths keyed by non LR-slice edge permutation and LR-slice edge permutation.
    #[serde(with = "serde_bytes")]
    edge: Vec<u8>,

    /// Depths keyed by the symmetry-reduced joint corner-permutation/non-LR-slice-edge-permutation coordinate.
    #[serde(with = "serde_bytes")]
    corner_edge_sym: Vec<u8>,
}

impl Table {
    /// Calculates and returns a new phase two pruning table.
    pub fn new() -> Table {
        phase_two()
    }

    /// Returns the number of moves the given cube is, from being solved.
    pub fn depth(&self, cube: &Cube) -> u8 {
        std::cmp::max(
            std::cmp::max(self.corner[corner_idx(cube)], self.edge[edge_idx(cube)]),
            self.corner_edge_sym[corner_edge_sym_idx(cube)],
        )
    }
}

/// Creates a new pattern database for phase two.
fn phase_two() -> Table {
    Table {
        corner: bfs(&PHASE_TWO_VALID_MOVES, DEPTH as u8, SIZE_CORNER, corner_idx, corner_idx),
        edge: bfs(&PHASE_TWO_VALID_MOVES, DEPTH as u8, SIZE_EDGE, edge_idx, edge_idx),
        corner_edge_sym: generate_corner_edge_sym(),
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

/// Returns the index within the symmetry-reduced combined corner/edge-permutation pruning table for the given
/// cube; see `super::symmetry`'s doc comment for how the reduction works.
fn corner_edge_sym_idx(cube: &Cube) -> usize {
    let raw = ptoidx(cube.corner_permutations());
    let (class, sym_idx) = CORNER_SYM[raw];
    let sym: &Symmetry = &SYMMETRIES[sym_idx as usize];

    class * NONSLICE_EDGE_PERM_STATES + ptoidx(&nonslice_edges(&sym.conjugate_edges(cube.edge_permutations())))
}

/// Extracts the four LR-slice edges (piece ids 8-11, guaranteed by phase one to occupy slots 8-11) and
/// relabels them to rank 0-3, for use with `ptoidx`.
fn slice_edges(perms: &[u8; NUM_EDGES]) -> [u8; 4] {
    std::array::from_fn(|i| perms[8 + i] - 8)
}

/// Extracts the eight non LR-slice edges (slots 0-7, already ranked 0-7 since piece ids 0-7 are contiguous),
/// for use with `ptoidx`.
fn nonslice_edges(perms: &[u8; NUM_EDGES]) -> [u8; 8] {
    std::array::from_fn(|i| perms[i])
}

/// Generates the `corner_edge_sym` pruning table via a dedicated, memory-light BFS.
///
/// `search::bfs`'s `HashSet`-based dedup and `Vec<Cube>` frontier don't scale to this table's ~112 million
/// entries (see `solver::kociemba::solver`'s doc comment) - both OOM, even on a 64GB machine. This BFS instead
/// tracks only `usize` coordinates: `dist` doubles as the visited set, and the frontier is a `Vec<u32>` of
/// indices rather than full `Cube`s. `Cube::redundant` filtering is skipped since it's an IDA*-only speed
/// optimization that BFS's `dist` dedup already subsumes for shortest-depth correctness.
fn generate_corner_edge_sym() -> Vec<u8> {
    let size = *NUM_CORNER_CLASSES * NONSLICE_EDGE_PERM_STATES;
    let sentinel = DEPTH as u8;

    let mut dist = vec![sentinel; size];

    let start = corner_edge_sym_idx(&Cube::new());
    dist[start] = 0;

    let mut frontier = vec![start as u32];

    for depth in 1..=(DEPTH as u8 - 1) {
        let mut next = Vec::new();

        for &idx in &frontier {
            expand_corner_edge_sym(idx, depth, &mut dist, sentinel, &mut next);
        }

        if next.is_empty() {
            break;
        }

        frontier = next;
    }

    dist
}

/// Decodes a `corner_edge_sym` table index back into a synthetic `Cube`: `REP_CORNER[class]`'s representative
/// corner permutation, `idx`'s non-slice edge permutation, and solved LR-slice edges/orientation.
///
/// Corner and edge permutations are independent group actions, so `Cube::rotate`'s permutation action is
/// correct on this synthetic embedding even though it isn't necessarily a reachable (or even legal) cube state;
/// see `symmetries_commute_with_phase_two_moves`, which relies on the same independence.
fn decode_corner_edge_sym(idx: usize) -> Cube {
    let class = idx / NONSLICE_EDGE_PERM_STATES;
    let edge_rank = idx % NONSLICE_EDGE_PERM_STATES;

    let cperms: [u8; NUM_CORNERS] = idxtoperm(REP_CORNER[class]);
    let nonslice: [u8; 8] = idxtoperm(edge_rank);
    let eperms: [u8; NUM_EDGES] = std::array::from_fn(|i| if i < 8 { nonslice[i] } else { i as u8 });

    Cube::from_perms(cperms, eperms)
}

/// Expands one BFS frontier index: applies every phase-two move to its decoded cube, recording `depth` for any
/// newly-discovered index in `dist` and appending it to `next`.
fn expand_corner_edge_sym(idx: u32, depth: u8, dist: &mut [u8], sentinel: u8, next: &mut Vec<u32>) {
    let cube = decode_corner_edge_sym(idx as usize);

    for &mv in PHASE_TWO_VALID_MOVES.iter() {
        let mut moved = cube;
        moved.rotate(mv);

        let nidx = corner_edge_sym_idx(&moved);

        if dist[nidx] == sentinel {
            dist[nidx] = depth;
            next.push(nidx as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::cube::permute;
    use crate::cube::Rotation;

    use super::*;

    /// Returns the corner/edge slot permutation induced by applying `m` to the solved cube; since moves act by a
    /// fixed permutation regardless of the current state, applying `m` from solved recovers that permutation
    /// directly (see `cube::cube::permute`'s "old[idx]/new[idx]" convention).
    fn move_perms(m: Rotation) -> ([u8; NUM_CORNERS], [u8; NUM_EDGES]) {
        let mut cube = Cube::new();
        cube.rotate(m);
        (*cube.corner_permutations(), *cube.edge_permutations())
    }

    /// Confirms the property that licenses reducing phase two's combined table by `SYMMETRIES`: conjugating any
    /// state, then applying any move in `PHASE_TWO_VALID_MOVES`, must be equivalent to applying some (symmetry
    /// dependent) move in `PHASE_TWO_VALID_MOVES` to the original state, then conjugating. If this doesn't hold,
    /// `CORNER_SYM`/`corner_edge_sym_idx`'s reduction is unsound.
    #[test]
    fn symmetries_commute_with_phase_two_moves() {
        // A handful of arbitrary corner/edge permutations - the property is a pure group-theoretic identity, so
        // it must hold for every permutation, not just cube-reachable ones.
        let sample_corners: [[u8; NUM_CORNERS]; 4] = [
            idxtoperm(0),
            idxtoperm(12345),
            idxtoperm(30000),
            idxtoperm(CORNER_PERM_STATES - 1),
        ];
        let sample_edges: [[u8; NUM_EDGES]; 4] = [
            idxtoperm(0),
            idxtoperm(1_234_567),
            idxtoperm(20_000_000),
            idxtoperm(factorial(NUM_EDGES) - 1),
        ];

        for sym in SYMMETRIES.iter() {
            for &m in PHASE_TWO_VALID_MOVES.iter() {
                let (corner_move, edge_move) = move_perms(m);

                for (corners, edges) in sample_corners.iter().zip(sample_edges.iter()) {
                    let moved_then_conjugated_corners = sym.conjugate_corners(&permute(*corners, corner_move));
                    let moved_then_conjugated_edges = sym.conjugate_edges(&permute(*edges, edge_move));

                    let conjugated_corners = sym.conjugate_corners(corners);
                    let conjugated_edges = sym.conjugate_edges(edges);

                    let found = PHASE_TWO_VALID_MOVES.iter().any(|&mp| {
                        let (corner_move_p, edge_move_p) = move_perms(mp);

                        permute(conjugated_corners, corner_move_p) == moved_then_conjugated_corners
                            && permute(conjugated_edges, edge_move_p) == moved_then_conjugated_edges
                    });

                    assert!(found, "symmetry {:?} does not commute with move {:?}", sym, m);
                }
            }
        }
    }
}
