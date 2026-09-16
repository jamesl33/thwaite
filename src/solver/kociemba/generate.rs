use crate::cube::{Cube, Rotation};

/// The number of distinct "last move face" variants a cube state can be found under: one for each of the six
/// faces, plus one for "no last move" (the initial state).
const VARIANTS: usize = 7;

/// Performs a breadth first search over `moves`, recording the depth (from the solved cube) of every distinct
/// coordinate produced by `idx`, up to `size` distinct coordinates.
///
/// A plain depth first search re-explores the same coordinate through every move sequence that reaches it,
/// which is exponential in the search depth (see `group_two`'s generation for the same reasoning). Breadth
/// first search visits coordinates in non-decreasing depth order, so the first time a coordinate is reached is
/// guaranteed to be its shortest depth; deduplicating on `(coordinate, last move face)` - rather than just
/// `coordinate`, since `redundant()`'s next-move eligibility depends on the last move's face - keeps the search
/// exhaustive while visiting each pair at most once.
///
/// `sentinel` seeds every entry; per Thistlewaite's own convention (see e.g. `group_zero::table::g0`), it's
/// expected to be the coordinate's true maximum depth, so any coordinate that's never visited (because it's
/// already at that maximum) is still left holding the correct value.
pub(super) fn bfs<F>(moves: &[Rotation], sentinel: usize, size: usize, idx: F) -> Vec<usize>
where
    F: Fn(&Cube) -> usize,
{
    let mut data = vec![sentinel; size];
    let mut visited = vec![false; size * VARIANTS];

    let start = Cube::new();

    data[idx(&start)] = 0;
    visited[variant(&start, &idx)] = true;

    let mut frontier = vec![start];

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
