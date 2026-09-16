use std::collections::HashSet;
use std::hash::Hash;

use crate::cube::{Cube, Rotation};

/// Performs a breadth first search over `moves`, starting from the solved cube, recording the depth of every
/// distinct coordinate produced by `idx`, up to `size` distinct coordinates. See `bfs_from` for the full
/// explanation; this is just its single-seed case.
pub(super) fn bfs<IF, KF, K>(moves: &[Rotation], sentinel: usize, size: usize, idx: IF, key: KF) -> Vec<usize>
where
    K: Eq + Hash,
    IF: Fn(&Cube) -> usize,
    KF: Fn(&Cube) -> K,
{
    bfs_from(moves, &[Cube::new()], sentinel, size, idx, key)
}

/// Performs a breadth first search over `moves`, starting from `seeds` (each at depth zero), recording the depth
/// of every distinct coordinate produced by `idx`, up to `size` distinct coordinates.
///
/// A plain depth first search re-explores the same coordinate through every move sequence that reaches it, which
/// is exponential in the search depth. Breadth first search visits coordinates in non-decreasing depth order, so
/// the first time a coordinate is reached is guaranteed to be its shortest depth; deduplicating on `(key,
/// last move face)` - rather than just `coordinate`, since `redundant()`'s next-move eligibility depends on the
/// last move's face - keeps the search exhaustive while visiting each pair at most once.
///
/// `idx` and `key` are often the same function, but don't have to be: `idx` is the (possibly coarse) coordinate
/// the resulting table is stored against, while `key` is what's used to decide if a state has already been
/// visited. `key` MUST be "closed" under the move action - i.e. any two cube states that produce the same `key`
/// are guaranteed to produce the same next `key` under any given move - otherwise this search silently misses
/// states, under-filling the table. `idx` alone often isn't closed, since it's frequently a lossy readout of just
/// part of the cube's state (e.g. edge orientation) whose evolution actually depends on other state (e.g. edge
/// permutation) that `idx` throws away. When that happens, fold the missing state into `key` (leaving `idx` as
/// the lossy coordinate the table still wants); see `group_zero::table::g0` for an example, where `key` pairs
/// the orientation coordinate with the full edge permutation rank.
///
/// `sentinel` seeds every entry; it's expected to be the coordinate's true maximum depth, so any coordinate
/// that's never visited (because it's already at that maximum) is still left holding the correct value.
///
/// Multiple seeds are needed where a single search from the solved cube can't reach every reachable coordinate
/// within a sane depth limit - see `group_two::table::initial`, which finds G2's 96 starting states this way.
pub(super) fn bfs_from<K, IF, KF>(
    moves: &[Rotation],
    seeds: &[Cube],
    sentinel: usize,
    size: usize,
    idx: IF,
    key: KF,
) -> Vec<usize>
where
    K: Eq + Hash,
    IF: Fn(&Cube) -> usize,
    KF: Fn(&Cube) -> K,
{
    let mut data = vec![sentinel; size];
    let mut visited = HashSet::new();

    for seed in seeds {
        data[idx(seed)] = 0;
        visited.insert((key(seed), face(seed.last())));
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

                if !visited.insert((key(&next_cube), face(next_cube.last()))) {
                    continue;
                }

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

/// Returns a stable index (`0..7`) for the face of the given move, or the "no move" slot for `None`.
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
