use std::hash::Hash;

use rustc_hash::FxHashSet;

use crate::cube::{Cube, Rotation};

/// Performs a breadth first search over `moves`, starting from the solved cube, recording the depth of every
/// distinct coordinate produced by `idx`, up to `size` distinct coordinates. See `bfs_from` for the full
/// explanation; this is just its single-seed case.
pub(super) fn bfs<IF, KF, K>(moves: &[Rotation], sentinel: u8, size: usize, idx: IF, key: KF) -> Vec<u8>
where
    K: Eq + Hash,
    IF: Fn(&Cube) -> usize,
    KF: Fn(&Cube) -> K,
{
    bfs_from(moves, &[Cube::new()], sentinel, size, idx, key)
}

/// Performs a breadth first search over `moves`, starting from `seeds` (each at depth zero), recording in the
/// returned table the depth at which every distinct coordinate produced by `idx` is first reached, up to `size`
/// distinct coordinates. `sentinel` seeds every entry, so a coordinate that's never visited (because it's already
/// at its true maximum depth, which `sentinel` is expected to be) is left holding the correct value.
///
/// `idx` and `key` are often the same function, but don't have to be: `idx` is the (possibly coarse) coordinate
/// the resulting table is stored against, while `key` is what's used to decide if a state has already been
/// visited. `key` MUST be "closed" under the move action - i.e. any two cube states that produce the same `key`
/// are guaranteed to produce the same next `key` under any given move - otherwise this search silently misses
/// states, under-filling the table. `idx` alone often isn't closed, since it's frequently a lossy readout of just
/// part of the cube's state that `idx` throws away; when that happens, fold the missing state into `key`, leaving
/// `idx` as the lossy coordinate the table still wants - see `group_zero::table::g0` for an example.
///
/// Multiple seeds are needed where a single search from the solved cube can't reach every reachable coordinate
/// within a sane depth limit - see `group_two::table::initial`, which finds G2's 96 starting states this way.
pub(super) fn bfs_from<K, IF, KF>(
    moves: &[Rotation],
    seeds: &[Cube],
    sentinel: u8,
    size: usize,
    idx: IF,
    key: KF,
) -> Vec<u8>
where
    K: Eq + Hash,
    IF: Fn(&Cube) -> usize,
    KF: Fn(&Cube) -> K,
{
    let mut data = vec![sentinel; size];
    // A plain depth first search re-explores the same coordinate through every move sequence that reaches it,
    // which is exponential in the search depth. Breadth first search visits coordinates in non-decreasing depth
    // order, so the first time a coordinate is reached is guaranteed to be its shortest depth; deduplicating on
    // `(key, last move face)` - rather than just `key` - keeps the search exhaustive while visiting each pair at
    // most once, since `redundant()`'s next-move eligibility depends on the last move's face.
    let mut visited = FxHashSet::default();

    for seed in seeds {
        data[idx(seed)] = 0;
        visited.insert((key(seed), face(seed.last())));
    }

    let mut frontier = seeds.to_vec();
    let mut ctx = Context {
        sentinel,
        data: &mut data,
        visited: &mut visited,
        idx: &idx,
        key: &key,
    };

    for depth in 1..=sentinel - 1 {
        let next = expand(&frontier, moves, depth, &mut ctx);

        if next.is_empty() {
            break;
        }

        frontier = next;
    }

    data
}

/// The search state threaded through each depth's frontier expansion: `data` and `visited` are mutated in place
/// as new coordinates are discovered, while `sentinel`, `idx` and `key` mirror `bfs_from`'s parameters of the
/// same name. Bundled into one struct so `expand` and `visit` don't need a long, easy-to-misorder argument list.
struct Context<'a, K, IF, KF> {
    sentinel: u8,
    data: &'a mut [u8],
    visited: &'a mut FxHashSet<(K, usize)>,
    idx: &'a IF,
    key: &'a KF,
}

/// Expands every cube in `frontier` by one move in each of `moves`, returning the newly discovered cubes.
///
/// For each surviving (non-redundant, not already visited) result, records its depth into `ctx.data` via
/// `ctx.idx` and marks it visited via `ctx.key` - see `visit` for the per-move details.
fn expand<K, IF, KF>(frontier: &[Cube], moves: &[Rotation], depth: u8, ctx: &mut Context<'_, K, IF, KF>) -> Vec<Cube>
where
    K: Eq + Hash,
    IF: Fn(&Cube) -> usize,
    KF: Fn(&Cube) -> K,
{
    let mut next = Vec::new();

    for cube in frontier {
        for mv in moves {
            if let Some(next_cube) = visit(cube, mv, depth, ctx) {
                next.push(next_cube);
            }
        }
    }

    next
}

/// Applies `mv` to `cube`, unless `redundant()` rules it out, returning the resulting cube if it's newly
/// discovered (recording its depth into `ctx.data` via `ctx.idx`, and marking it visited via `ctx.key`), or
/// `None` if `mv` was skipped or the result was already visited.
fn visit<K, IF, KF>(cube: &Cube, mv: &Rotation, depth: u8, ctx: &mut Context<'_, K, IF, KF>) -> Option<Cube>
where
    K: Eq + Hash,
    IF: Fn(&Cube) -> usize,
    KF: Fn(&Cube) -> K,
{
    if cube.redundant(mv) {
        return None;
    }

    let mut next_cube = *cube;
    next_cube.rotate(*mv);

    if !ctx.visited.insert(((ctx.key)(&next_cube), face(next_cube.last()))) {
        return None;
    }

    let i = (ctx.idx)(&next_cube);

    if ctx.data[i] == ctx.sentinel {
        ctx.data[i] = depth;
    }

    Some(next_cube)
}

/// Returns a stable index (`0..7`) for the face of the given move (using `Rotation::face()` to collapse the
/// clockwise/counter-clockwise/180 variants of a face to the same index), or the "no move" slot for `None`.
fn face(mv: Option<Rotation>) -> usize {
    let Some(mv) = mv else {
        return 0;
    };

    match mv.face() {
        Rotation::U => 1,
        Rotation::D => 2,
        Rotation::L => 3,
        Rotation::R => 4,
        Rotation::F => 5,
        Rotation::B => 6,
        _ => unreachable!("Rotation::face() always returns a base face rotation"),
    }
}
