use crate::cube::{Cube, Rotation};

/// The number of distinct "last move face" variants a cube state can be found under: one for each of the six
/// faces, plus one for "no last move" (the initial state).
const VARIANTS: usize = 7;

/// Performs a breadth first search over `moves`, starting from the solved cube, recording the depth of every
/// distinct coordinate produced by `idx`, up to `size` distinct coordinates. See `bfs_seeded` for the full
/// explanation; this is just its single-seed case.
pub(super) fn bfs<F>(moves: &[Rotation], sentinel: usize, size: usize, idx: F) -> Vec<usize>
where
    F: Fn(&Cube) -> usize,
{
    bfs_seeded(moves, &[Cube::new()], sentinel, size, idx)
}

/// Performs a breadth first search over `moves`, starting from `seeds` (each at depth zero), recording the depth
/// of every distinct coordinate produced by `idx`, up to `size` distinct coordinates.
///
/// A plain depth first search re-explores the same coordinate through every move sequence that reaches it, which
/// is exponential in the search depth. Breadth first search visits coordinates in non-decreasing depth order, so
/// the first time a coordinate is reached is guaranteed to be its shortest depth; deduplicating on `(coordinate,
/// last move face)` - rather than just `coordinate`, since `redundant()`'s next-move eligibility depends on the
/// last move's face - keeps the search exhaustive while visiting each pair at most once.
///
/// `sentinel` seeds every entry; it's expected to be the coordinate's true maximum depth, so any coordinate
/// that's never visited (because it's already at that maximum) is still left holding the correct value.
///
/// Multiple seeds are needed where a single search from the solved cube can't reach every reachable coordinate
/// within a sane depth limit - see `group_two::table::initial`, which finds G2's 96 starting states this way.
///
/// CAUTION: deduplicating on `(coordinate, last move face)` is only correct if `idx` captures enough of the cube
/// state that two states sharing a coordinate are guaranteed to transition to the same next coordinate under any
/// given move - i.e. `idx` must be "closed" under the move action, not just a compressed *readout* of state that
/// happens to coincide for two otherwise-different cubes. This holds for G2, G3 and both Kociemba phases (their
/// coordinates combine enough permutation/combination information to be closed), but does NOT hold for a bare
/// orientation-only coordinate like G0's or G1's partial-permutation one - using this function for those silently
/// under-fills the table (confirmed by comparing against the old exhaustive depth first search's output), since
/// two cube states with identical orientation but different underlying permutation can reach different follow-up
/// coordinates under the same move. G0 and G1 stay on the older, more expensive but always-correct depth first
/// search with a `cmp::min` reduction (see `group_zero::table::g0`) until/unless this is fixed to track enough
/// extra state (e.g. deduplicating on full cube state rather than just `idx`) to be safe for them too.
pub(super) fn bfs_seeded<F>(moves: &[Rotation], seeds: &[Cube], sentinel: usize, size: usize, idx: F) -> Vec<usize>
where
    F: Fn(&Cube) -> usize,
{
    let mut data = vec![sentinel; size];
    let mut visited = vec![false; size * VARIANTS];

    for seed in seeds {
        data[idx(seed)] = 0;
        visited[variant(seed, &idx)] = true;
    }

    let mut frontier = seeds.to_vec();

    for depth in 1..=sentinel - 1 {
        let mut next = Vec::new();

        for cube in &frontier {
            for mv in moves {
                if cube.redundant(mv) {
                    continue;
                }

                let mut next_cube = *cube;
                next_cube.rotate(*mv);

                let v = variant(&next_cube, &idx);

                if visited[v] {
                    continue;
                }

                visited[v] = true;

                let i = idx(&next_cube);

                if data[i] == sentinel {
                    data[i] = depth;
                }

                next.push(next_cube);
            }
        }

        if next.is_empty() {
            break;
        }

        frontier = next;
    }

    data
}

/// Returns the `(coordinate, last move face)` variant index for the given cube.
fn variant<F>(cube: &Cube, idx: &F) -> usize
where
    F: Fn(&Cube) -> usize,
{
    idx(cube) * VARIANTS + face(cube.last())
}

/// Returns a stable index (`0..VARIANTS`) for the face of the given move, or the "no move" slot for `None`.
fn face(mv: Option<Rotation>) -> usize {
    match mv.map(|mv| mv.face()) {
        None => 0,
        Some(Rotation::U) => 1,
        Some(Rotation::D) => 2,
        Some(Rotation::L) => 3,
        Some(Rotation::R) => 4,
        Some(Rotation::F) => 5,
        Some(Rotation::B) => 6,
        Some(_) => unreachable!("Rotation::face() always returns a base face rotation"),
    }
}
