use serde::{Deserialize, Serialize};

use crate::cube::{Cube, CORNER_ORIENTATIONS, EDGE_ORIENTATIONS, NUM_CORNERS, NUM_EDGES};
use crate::solver::generate::bfs;
use crate::solver::kociemba::phase::PHASE_ONE_VALID_MOVES;
use crate::solver::maths::combinations;

/// The number of combinations of the four LR-slice (no L/R facelet, piece ids 8-11) edge pieces amongst all
/// twelve edge slots.
const LRSLICE_COMBINATIONS: usize = combinations(NUM_EDGES, 4);

/// The number of states for the corner orientations, which are being fixed in phase one.
const CORNER_ORIENTATION_STATES: usize = usize::pow(CORNER_ORIENTATIONS, (NUM_CORNERS - 1) as u32);

/// The number of states for the edge orientations, which are being fixed in phase one.
const EDGE_ORIENTATION_STATES: usize = usize::pow(EDGE_ORIENTATIONS, (NUM_EDGES - 1) as u32);

/// The size of the corner-orientation pruning table, a flat two dimensional array.
const SIZE_CORNER: usize = CORNER_ORIENTATION_STATES * LRSLICE_COMBINATIONS;

/// The size of the edge-orientation pruning table, a flat two dimensional array.
const SIZE_EDGE: usize = EDGE_ORIENTATION_STATES * LRSLICE_COMBINATIONS;

/// As documented by Kociemba, phase one requires at most twelve moves.
///
/// https://kociemba.org/cube.htm
const DEPTH: usize = 12;

/// The size of the lookup table for the LR-slice edge combinations.
const IDX_LOOKUP_TABLE_SIZE: usize = 2048;

/// Translates an index in the range 0-2048 into 0-495, allowing us to treat the LR-slice edge combination as a
/// binary number.
const IDX_LOOKUP_TABLE: [usize; IDX_LOOKUP_TABLE_SIZE] = idx_lookup_table();

/// The pruning table for phase one; the depth to reach phase one's target group is the maximum of the two
/// coordinates, since together they'd be too large (~2.2 billion entries) to store densely.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// Depths keyed by corner-orientation and LR-slice edge combination.
    corner: Vec<u8>,

    /// Depths keyed by edge-orientation and LR-slice edge combination.
    edge: Vec<u8>,
}

impl Table {
    /// Calculates and returns a new phase one pruning table.
    pub fn new() -> Table {
        phase_one()
    }

    /// Returns the number of moves the given cube is, from being in phase one's target group.
    pub fn depth(&self, cube: &Cube) -> u8 {
        std::cmp::max(self.corner[corner_idx(cube)], self.edge[edge_idx(cube)])
    }
}

/// Creates a new pattern database for phase one.
///
/// CAUTION: `corner_idx` and `edge_idx` each combine an orientation coordinate with the LR slice edge combination;
/// the orientation part isn't closed under the move action on its own - it depends on the matching permutation too
/// (see `crate::cube::Cube::rotate_up`) - so this search can silently miss states (see
/// `crate::solver::generate::bfs_from`'s doc comment). Pairing the search key with the matching permutation rank
/// fixes this, but the resulting key space is large enough (corner permutation rank alone is 8! = 40320, crossed
/// with ~1e6 `corner_idx` values) that it OOMs in practice; the same problem hit Thistlewaite's G1 (see
/// `group_one::table::g1`, which uses an exhaustive depth first search instead). Left unfixed here since Kociemba's
/// solver is still an unwired MVP.
fn phase_one() -> Table {
    Table {
        corner: bfs(&PHASE_ONE_VALID_MOVES, DEPTH as u8, SIZE_CORNER, corner_idx, corner_idx),
        edge: bfs(&PHASE_ONE_VALID_MOVES, DEPTH as u8, SIZE_EDGE, edge_idx, edge_idx),
    }
}

/// Calculates the LR-slice edge combination lookup table, noting that there's 495 variations of eleven digit
/// binary numbers where there's three/four ones.
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

/// Returns the index within the corner-orientation pruning table for the given cube.
fn corner_idx(cube: &Cube) -> usize {
    otoidx(&slot_corner_orientations(cube)) * LRSLICE_COMBINATIONS + lrslice_ctoidx(cube.edge_permutations())
}

/// Returns the index within the edge-orientation pruning table for the given cube.
fn edge_idx(cube: &Cube) -> usize {
    eotoidx(&slot_edge_orientations(cube)) * LRSLICE_COMBINATIONS + lrslice_ctoidx(cube.edge_permutations())
}

/// Returns corner orientation re-indexed by slot rather than piece id: `[i]` is the orientation of whichever
/// piece currently occupies slot `i`.
///
/// `Cube::corner_orientations()` is indexed by piece id, so its evolution under a move depends on
/// `Cube::corner_permutations()` too (a move's fixed per-slot twist lands on whichever piece is in that slot) -
/// re-indexing by slot here removes that dependency, since a move's twist is then applied at a fixed slot
/// regardless of which piece occupies it. This is what actually closes the coordinate under the move action
/// (verified empirically: bucketing legally-reachable cubes by this slot-indexed vector and comparing each
/// bucket's post-move results gives zero mismatches, where the same test on the raw piece-indexed vector
/// mismatches on the large majority of buckets - i.e. the piece-indexed table this code shipped with was
/// silently non-admissible).
fn slot_corner_orientations(cube: &Cube) -> [usize; NUM_CORNERS] {
    let orien = cube.corner_orientations();
    let perms = cube.corner_permutations();
    std::array::from_fn(|i| orien[perms[i]])
}

/// Returns edge orientation re-indexed by slot rather than piece id; see `slot_corner_orientations`, whose
/// reasoning applies identically here.
fn slot_edge_orientations(cube: &Cube) -> [usize; NUM_EDGES] {
    let orien = cube.edge_orientations();
    let perms = cube.edge_permutations();
    std::array::from_fn(|i| orien[perms[i]])
}

/// Returns the index for the given corner orientations, treating them as a base three number.
///
/// https://www.jaapsch.net/puzzles/compindx.htm#orient
fn otoidx<const N: usize>(orien: &[usize; N]) -> usize {
    let mut idx: usize = 0;

    for i in 0..N - 1 {
        idx = idx * CORNER_ORIENTATIONS + orien[i]
    }

    debug_assert!(idx < CORNER_ORIENTATION_STATES);

    idx
}

/// Returns the index for the given edge orientations, treating them as a base two number.
///
/// https://www.jaapsch.net/puzzles/compindx.htm#orient
fn eotoidx(orien: &[usize; NUM_EDGES]) -> usize {
    let mut idx: usize = 0;

    for i in 0..NUM_EDGES - 1 {
        idx = idx * EDGE_ORIENTATIONS + orien[i]
    }

    debug_assert!(idx < EDGE_ORIENTATION_STATES);

    idx
}

/// Returns the index for the LR-slice edges (piece ids 8-11, the edges with no L/R facelet - the same edges
/// Thistlewaite's own G1 tracks, since this codebase defines orientation relative to the L/R axis rather than
/// the textbook Kociemba U/D axis; see `phase::PHASE_TWO_VALID_MOVES`), calculated by treating their occupied
/// slots as a binary number (ignoring the last slot) then converting that into a number between 0-495 using a
/// lookup table.
fn lrslice_ctoidx(perms: &[usize; NUM_EDGES]) -> usize {
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        if perms[i] <= 7 {
            continue;
        }

        dec += 2_usize.pow(10 - i as u32);
    }

    let idx = IDX_LOOKUP_TABLE[dec];

    debug_assert!(idx < LRSLICE_COMBINATIONS);

    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms the property `corner_idx`/`edge_idx` rely on for `bfs`'s dedup to be sound: two reachable cubes
    /// with the same slot-indexed orientation must land on the same slot-indexed orientation after any given
    /// move, regardless of how their underlying permutations differ. This is what actually closes the
    /// coordinate under the move action - the piece-indexed `Cube::corner_orientations()`/`edge_orientations()`
    /// this codebase shipped with initially does NOT have this property (a move's fixed per-slot twist lands on
    /// whichever piece occupies that slot, so the piece-indexed vector's evolution depends on the permutation
    /// too), which silently made the pruning table non-admissible.
    #[test]
    fn slot_indexed_orientation_is_closed_under_moves() {
        let mut buckets: std::collections::HashMap<([usize; NUM_CORNERS], [usize; NUM_EDGES]), Vec<Cube>> =
            std::collections::HashMap::new();

        let mut cube = Cube::new();
        let mut seed = 1u64;

        for _ in 0..20_000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mv = PHASE_ONE_VALID_MOVES[(seed >> 33) as usize % PHASE_ONE_VALID_MOVES.len()];
            cube.rotate(mv);

            let key = (slot_corner_orientations(&cube), slot_edge_orientations(&cube));
            buckets.entry(key).or_default().push(cube);
        }

        for states in buckets.values() {
            if states.len() < 2 {
                continue;
            }

            for &mv in PHASE_ONE_VALID_MOVES.iter() {
                let mut results = states.iter().map(|c| {
                    let mut c = *c;
                    c.rotate(mv);
                    (slot_corner_orientations(&c), slot_edge_orientations(&c))
                });

                let first = results.next().unwrap();

                assert!(
                    results.all(|r| r == first),
                    "slot-indexed orientation coordinate isn't closed under move {:?}",
                    mv
                );
            }
        }
    }
}
