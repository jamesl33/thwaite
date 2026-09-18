use std::cmp;

use crate::cube::{Cube, Rotation};

/// Perform an iterative deepening A* search, using the given heuristic.
pub fn idas<F>(cube: Cube, moves: &[Rotation], hueristic: &F) -> Option<Vec<Rotation>>
where
    F: Fn(&Cube) -> u8,
{
    let mut limit = hueristic(&cube);

    // Already in the target group, exit early
    if limit == 0 {
        return Some(vec![]);
    }

    loop {
        let (t, path) = idas_dfs(cube, 0, limit, moves, hueristic);

        // dfs builds the path in reverse (deepest move first), each level appending rather than prepending its own
        // move, to keep path reconstruction O(depth) instead of O(depth^2). Restore solution order.
        if let Some(mut path) = path {
            return Some({ path.reverse(); path });
        }

        if t == u8::MAX {
            return None;
        }

        limit = t;
    }
}

/// Perform a depth first search, using the given moves and heuristic returning the minimum cost branch and the moves
/// that have been made to get there, in reverse order (deepest move first) - see `idas`'s comment on why.
fn idas_dfs<F>(cube: Cube, g: u8, limit: u8, valid: &[Rotation], hueristic: &F) -> (u8, Option<Vec<Rotation>>)
where
    F: Fn(&Cube) -> u8,
{
    let mut min = u8::MAX;

    for mv in valid {
        if cube.redundant(mv) {
            continue;
        }

        let mut cube = cube;

        cube.rotate(*mv);

        let h = hueristic(&cube);
        let f = g + h;

        if h == 0 {
            return (0, Some(vec![*mv]));
        }

        if f > limit {
            min = cmp::min(min, f);
            continue;
        }

        let (cost, path) = idas_dfs(cube, g + 1, limit, valid, hueristic);

        if let Some(mut path) = path {
            return (0, Some({ path.push(*mv); path }));
        }

        min = cmp::min(min, cost);
    }

    (min, None)
}

/// Performs a depth first search, applying the given `moves` up to `limit`, running `func` for every cube state
/// visited (not just goal states) - the counterpart to `generate::bfs`/`generate::bfs_from`, for callers that want
/// every visited state rather than just per-coordinate depths, e.g. group one and two's pattern-database
/// generation.
pub fn dfs<F>(cube: Cube, moves: &[Rotation], limit: usize, func: &mut F)
where
    F: FnMut(&Cube, usize),
{
    dfs_visit(cube, moves, 1, limit, func);
}

/// The recursive step behind `dfs`.
fn dfs_visit<F>(cube: Cube, moves: &[Rotation], depth: usize, limit: usize, func: &mut F)
where
    F: FnMut(&Cube, usize),
{
    for mv in moves {
        if cube.redundant(mv) {
            continue;
        }

        let mut cube = cube;

        cube.rotate(*mv);

        func(&cube, depth);

        // We've reached our limit, stop searching
        if depth >= limit {
            continue;
        }

        dfs_visit(cube, moves, depth + 1, limit, func);
    }
}
