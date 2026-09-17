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

/// The pruning table for phase two; the depth to solve the cube (having already reached phase one's target
/// group) is the maximum of three coordinates. `corner` and `edge` are weakly correlated (a known MVP gap - see
/// `solver::kociemba::solver`'s doc comment history), so `corner_edge_sym` adds a genuinely joint corner/edge
/// coordinate, made tractable to store by reducing it with `SYMMETRIES`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// Depths keyed by corner permutation and LR-slice edge permutation.
    corner: Vec<usize>,

    /// Depths keyed by non LR-slice edge permutation and LR-slice edge permutation.
    edge: Vec<usize>,

    /// Depths keyed by the symmetry-reduced joint corner-permutation/non-LR-slice-edge-permutation coordinate.
    corner_edge_sym: Vec<u8>,
}

impl Table {
    /// Calculates and returns a new phase two pruning table.
    pub fn new() -> Table {
        phase_two()
    }

    /// Returns the number of moves the given cube is, from being solved.
    pub fn depth(&self, cube: &Cube) -> usize {
        std::cmp::max(
            std::cmp::max(self.corner[corner_idx(cube)], self.edge[edge_idx(cube)]),
            self.corner_edge_sym[corner_edge_sym_idx(cube)] as usize,
        )
    }
}

/// Creates a new pattern database for phase two.
fn phase_two() -> Table {
    let size_corner_edge_sym = *NUM_CORNER_CLASSES * NONSLICE_EDGE_PERM_STATES;

    Table {
        corner: bfs(&PHASE_TWO_VALID_MOVES, DEPTH, SIZE_CORNER, corner_idx, corner_idx),
        edge: bfs(&PHASE_TWO_VALID_MOVES, DEPTH, SIZE_EDGE, edge_idx, edge_idx),
        corner_edge_sym: bfs(
            &PHASE_TWO_VALID_MOVES,
            DEPTH,
            size_corner_edge_sym,
            corner_edge_sym_idx,
            corner_edge_sym_idx,
        )
        .into_iter()
        .map(|depth| depth as u8)
        .collect(),
    }
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
