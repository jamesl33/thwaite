use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cube::{Cube, Rotation, NUM_CORNERS, NUM_EDGES};
use crate::solver::group::Group;
use crate::solver::maths::{combinations, factorial};

/// There's 96 starting states for G2.
const INITIAL: usize = 96;

/// The number of corner permutations that we're fixing in this group.
const N_SIZE: usize = factorial(8);

/// The number of edge distributions that we're fixing in this group.
const M_SIZE: usize = combinations(8, 4);

/// The size of the G2 pruning table, which is a flat two dimensional array.
const SIZE: usize = N_SIZE * M_SIZE;

/// The pruning table for the G2.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// The underlying data, where each index represents a cube state and its depth from the solved state.
    data: Vec<usize>,
}

impl Table {
    /// Calculates and returns a new G2 pruning table.
    pub fn new() -> Table {
        g2()
    }

    /// Returns the number of moves the given cube is, from being in G2.
    pub fn depth(&self, cube: &Cube) -> usize {
        self.data[idx(cube)]
    }
}

/// The number of distinct "last move face" variants a cube state can be found under: one for each of the six
/// faces, plus one for "no last move" (the initial states).
const VARIANTS: usize = 7;

// As documented the max depth from G2 is thirteen.
///
/// http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf
const DEPTH: usize = 13;

/// Creates a new pattern database for G2.
fn g2() -> Table {
    let mut tab: Table = Table {
        data: vec![DEPTH; SIZE],
    };

    // Tracks which (state, last move face) variants have already been expanded.
    //
    // `redundant` decides which moves are allowed next based on the face of the last move applied, not just the
    // cube's state - so two paths that reach the same state via a different last move face can have different
    // sets of moves available to them. Deduping purely on state (as opposed to `(state, last move face)`) would
    // discard whichever variant arrives second, which can permanently cut off the shortest path to some
    // descendant state. Expanding every variant once keeps the search exhaustive while still visiting each
    // (state, last move face) pair at most once.
    let mut visited = vec![false; SIZE * VARIANTS];

    // Generate the 96 starting states, one per corner-permutation orbit; these are our depth zero.
    let mut frontier = initial();

    for cube in &frontier {
        tab.data[idx(cube)] = 0;
        visited[variant(cube)] = true;
    }

    // Perform a breadth first search outward from the 96 starting states, applying all the valid G2 moves.
    //
    // A plain depth first search re-explores the same cube states through every move sequence that reaches
    // them, which is exponential in the search depth. Since BFS visits states in non-decreasing depth order,
    // the first time a state is reached is guaranteed to be its shortest depth, so each reachable state only
    // needs to be expanded once - turning the search from exponential into roughly `states * branching factor`.
    for depth in 1..=DEPTH - 1 {
        let mut next = Vec::new();

        for cube in &frontier {
            expand(cube, depth, &mut tab, &mut visited, &mut next);
        }

        if next.is_empty() {
            break;
        }

        frontier = next;
    }

    tab
}

/// Applies every valid G2 move to `cube`, recording newly discovered states at `depth` in `tab` and queuing them
/// in `next`. Moves whose `(state, last move face)` variant was already visited are skipped.
fn expand(cube: &Cube, depth: usize, tab: &mut Table, visited: &mut [bool], next: &mut Vec<Cube>) {
    for mv in Group::Two.moves() {
        if cube.redundant(mv) {
            continue;
        }

        let mut next_cube = *cube;
        next_cube.rotate(*mv);

        let v = variant(&next_cube);

        if visited[v] {
            continue;
        }

        visited[v] = true;

        let idx = idx(&next_cube);

        if tab.data[idx] == DEPTH {
            tab.data[idx] = depth;
        }

        next.push(next_cube);
    }
}

/// Returns the `(state, last move face)` variant index for the given cube, for use with the `visited` array.
fn variant(cube: &Cube) -> usize {
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

/// Returns the index in the pruning table for the given cube.
fn idx(cube: &Cube) -> usize {
    // Calculate the index for the corner permutations
    let cpidx = ptoidx(cube.corner_permutations());

    // Calculate the index for the edge permutations
    let epidx = pctoidx::<8>(cube.edge_permutations()[..NUM_EDGES - 4].try_into().unwrap());

    // Calculate the index in the flattened two-dimensional array
    cpidx * M_SIZE + epidx
}

/// Returns the 96 initial cube states for G2.
fn initial() -> Vec<Cube> {
    // The depth required to generate all 96 initial states in G2
    const DEPTH: usize = 5;

    // Where we'll store the initial states
    let mut cubes = HashMap::<usize, Cube>::new();

    // We start searching from a solved cube
    let start: Cube = Cube::new();

    // Perform a depth first search, applying all the valid G2 moves and recording the depth from the solved state
    start.search(Group::Two.moves(), DEPTH - 1, &mut |cube, _| {
        // If the corners aren't in orbit, we ignore the cube state (as it's not a valid initial state for G2)
        if !cino(cube.corner_permutations()) {
            return;
        }

        // Calculate the index for the corner permutations
        let idx = ptoidx(cube.corner_permutations());

        // If we've already found this cube, exit early
        if cubes.contains_key(&idx) {
            return;
        }

        // We've found a valid initial cube state, store it
        cubes.insert(idx, *cube);
    });

    cubes.into_values().collect()
}

/// Returns a boolean indicating whether the provided corners are in orbit.
fn cino(cperms: &[usize; NUM_CORNERS]) -> bool {
    for i in 0..4 {
        if cperms[i] > 3 {
            return false;
        }
    }

    true
}

/// Returns the index in the pruning table for the given corner permutations.
///
/// https://www.jaapsch.net/puzzles/compindx.htm#perm
pub fn ptoidx<const N: usize>(perms: &[usize; N]) -> usize {
    let mut t = 0;

    for i in 0..N - 1 {
        t *= N - i;

        for j in i + 1..N {
            if perms[i] > perms[j] {
                t += 1;
            }
        }
    }

    t
}

/// Returns the index in the pruning table for the given edge permutation combinations.
///
/// https://www.jaapsch.net/puzzles/compindx.htm#comb
fn pctoidx<const N: usize>(perms: &[usize; N]) -> usize {
    let mut t = 0;
    let mut r = 4;

    for i in (0..N).rev() {
        if perms[i] <= 3 {
            continue;
        }

        t += combinations(i, r);
        r -= 1;
    }

    debug_assert!(t < M_SIZE);

    t
}
