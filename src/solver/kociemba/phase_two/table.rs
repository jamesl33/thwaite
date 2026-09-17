use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::cube::{Cube, Symmetry, NUM_CORNERS, NUM_EDGES, SYMMETRIES};
use crate::solver::generate::bfs;
use crate::solver::kociemba::phase::PHASE_TWO_VALID_MOVES;
use crate::solver::maths::{factorial, idxtoperm, ptoidx};

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

/// For each raw corner-permutation rank (0..`CORNER_PERM_STATES`): the dense id of its symmetry orbit under
/// `SYMMETRIES`, and which of the 16 symmetries maps this raw state to that orbit's chosen representative.
///
/// This is the standard trick that lets the combined corner/edge table below be indexed without a 16-way search
/// at lookup time: canonicalizing the (much smaller) corner-permutation coordinate alone tells us exactly which
/// symmetry to also apply to the edge-permutation coordinate.
static CORNER_SYM: LazyLock<Vec<(usize, u8)>> = LazyLock::new(|| {
    let mut table: Vec<Option<(usize, u8)>> = vec![None; CORNER_PERM_STATES];
    let mut next_class = 0;

    for raw in 0..CORNER_PERM_STATES {
        if table[raw].is_some() {
            continue;
        }

        let perm: [usize; NUM_CORNERS] = idxtoperm(raw);

        let orbit: Vec<usize> = SYMMETRIES
            .iter()
            .map(|sym| ptoidx(&sym.conjugate_corners(&perm)))
            .collect();

        let representative = *orbit.iter().min().unwrap();

        for &member in &orbit {
            if table[member].is_some() {
                continue;
            }

            let member_perm: [usize; NUM_CORNERS] = idxtoperm(member);

            let sym_idx = SYMMETRIES
                .iter()
                .position(|sym| ptoidx(&sym.conjugate_corners(&member_perm)) == representative)
                .expect("every orbit member must map to its representative under some symmetry");

            table[member] = Some((next_class, sym_idx as u8));
        }

        next_class += 1;
    }

    table.into_iter().map(|entry| entry.unwrap()).collect()
});

/// The number of distinct corner-permutation symmetry classes, i.e. the number of orbits `CORNER_SYM` assigns.
static NUM_CORNER_CLASSES: LazyLock<usize> = LazyLock::new(|| CORNER_SYM.iter().map(|&(class, _)| class).max().unwrap() + 1);

/// For each corner symmetry class: the raw corner-permutation rank chosen as that orbit's representative.
///
/// `SYMMETRIES[0]` is always the identity (see `build_symmetries`'s `group = vec![Symmetry::IDENTITY]` seed, which
/// is never reordered), so identity is the only symmetry that can map a raw state to itself - meaning `CORNER_SYM`
/// records `sym_idx == 0` exactly for each class's representative, and nowhere else.
static REP_CORNER: LazyLock<Vec<usize>> = LazyLock::new(|| {
    let mut rep = vec![0; *NUM_CORNER_CLASSES];

    for (raw, &(class, sym_idx)) in CORNER_SYM.iter().enumerate() {
        if sym_idx == 0 {
            rep[class] = raw;
        }
    }

    rep
});

/// The pruning table for phase two; the depth to solve the cube (having already reached phase one's target
/// group) is the maximum of three coordinates. `corner` and `edge` are only weakly correlated, so
/// `corner_edge_sym` adds a genuinely joint corner/edge coordinate, made tractable to store by reducing it with
/// `SYMMETRIES`.
///
/// `corner_edge_sym` doesn't make `corner`/`edge` redundant: it never sees LR-slice-edge permutation at all
/// (`corner_edge_sym_idx` drops it entirely), while `corner`/`edge` each pair their piece-type with it - they're
/// the only signal for states where slice-edge permutation is the dominant remaining work. Measured empirically
/// by sampling 500,000 reachable states against the checked-in table: `corner` alone exceeds `corner_edge_sym`
/// in ~38% of them (by up to 8 moves), `edge` alone in ~38% (by up to 9 moves) - dropping either would
/// meaningfully weaken the heuristic, not just simplify bookkeeping.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// Depths keyed by corner permutation and LR-slice edge permutation.
    corner: Vec<u8>,

    /// Depths keyed by non LR-slice edge permutation and LR-slice edge permutation.
    edge: Vec<u8>,

    /// Depths keyed by the symmetry-reduced joint corner-permutation/non-LR-slice-edge-permutation coordinate.
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

/// Generates the `corner_edge_sym` pruning table via a dedicated, memory-light BFS.
///
/// `generate::bfs`'s `HashSet<(key, face)>` dedup and `Vec<Cube>` frontier don't scale to this table's ~112
/// million entries (see `solver::kociemba::solver`'s doc comment) - both OOM, even on a 64GB machine. This BFS
/// instead works purely on `usize` coordinates: the `dist` array doubles as the visited set (no separate hash
/// set), and the frontier is a `Vec<u32>` of indices rather than full `Cube`s.
///
/// Each frontier index is decoded back into a synthetic `Cube` (via `REP_CORNER`'s representative corner
/// permutation, this index's edge permutation, and solved LR-slice edges/orientation) purely so `Cube::rotate` can
/// compute the move's permutation action - corner and edge permutations are independent group actions, so this
/// synthetic embedding doesn't need to be a reachable (or even legal) cube state for that action to be correct;
/// see `symmetries_commute_with_phase_two_moves`, which relies on the same independence.
///
/// `Cube::redundant` filtering is intentionally skipped: it's a search-speed optimization for IDA*, not a
/// requirement for BFS's shortest-depth correctness (a "redundant" move only ever reaches a state some shorter,
/// non-redundant sequence already reached, so `dist`'s dedup discards it just as effectively).
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
            let idx = idx as usize;
            let class = idx / NONSLICE_EDGE_PERM_STATES;
            let edge_rank = idx % NONSLICE_EDGE_PERM_STATES;

            let cperms: [usize; NUM_CORNERS] = idxtoperm(REP_CORNER[class]);
            let nonslice: [usize; 8] = idxtoperm(edge_rank);
            let eperms: [usize; NUM_EDGES] = std::array::from_fn(|i| if i < 8 { nonslice[i] } else { i });

            let cube = Cube::from_perms(cperms, eperms);

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

        if next.is_empty() {
            break;
        }

        frontier = next;
    }

    dist
}

/// Returns the index within the symmetry-reduced combined corner/edge-permutation pruning table for the given
/// cube; see `CORNER_SYM`'s doc comment for how the reduction works.
fn corner_edge_sym_idx(cube: &Cube) -> usize {
    let raw = ptoidx(cube.corner_permutations());
    let (class, sym_idx) = CORNER_SYM[raw];
    let sym: &Symmetry = &SYMMETRIES[sym_idx as usize];

    class * NONSLICE_EDGE_PERM_STATES + ptoidx(&nonslice_edges(&sym.conjugate_edges(cube.edge_permutations())))
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

#[cfg(test)]
mod tests {
    use crate::cube::cube::permute;
    use crate::cube::Rotation;

    use super::*;

    /// Returns the corner/edge slot permutation induced by applying `m` to the solved cube; since moves act by a
    /// fixed permutation regardless of the current state, applying `m` from solved recovers that permutation
    /// directly (see `cube::cube::permute`'s "old[idx]/new[idx]" convention).
    fn move_perms(m: Rotation) -> ([usize; NUM_CORNERS], [usize; NUM_EDGES]) {
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
        let sample_corners: [[usize; NUM_CORNERS]; 4] = [
            idxtoperm(0),
            idxtoperm(12345),
            idxtoperm(30000),
            idxtoperm(CORNER_PERM_STATES - 1),
        ];
        let sample_edges: [[usize; NUM_EDGES]; 4] = [
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
