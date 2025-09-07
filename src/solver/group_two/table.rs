use std::cmp;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cube::{Cube, NUM_CORNERS, NUM_EDGES};
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

/// Creates a new pattern database for G2.
///
/// TODO (jamesl33): This search could be done in parallel.
fn g2() -> Table {
    // As documented the max depth from G2 is 13, however, this takes far too long to generate; 10 seems sufficient.
    //
    // http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf
    const DEPTH: usize = 10;

    // Create the pruning table, ready to be populated
    let mut tab: Table = Table {
        data: vec![DEPTH; SIZE],
    };

    // Generate the 96 starting states
    let cubes = initial();

    // Perform a depth first search for each starting state, applying all the valid G2 moves and recording the depth
    // from the solved state.
    for cube in cubes {
        dfs(&mut tab, cube, DEPTH - 1)
    }

    tab
}

/// Performs a depth first search from the given cube state.
fn dfs(tab: &mut Table, cube: Cube, limit: usize) {
    // The zeroth index represents the solved state (e.g. in G2)
    update(tab, &cube, 0);

    // Perform a depth first search, applying all the valid G2 moves and recording the depth from the solved state
    cube.search(Group::Two.moves(), limit, &mut |cube, depth| update(tab, cube, depth));
}

/// Populates the depth in the pruning table - if it's less that the existing depth - for the given cube.
fn update(tab: &mut Table, cube: &Cube, depth: usize) {
    // Calculate the index in the pruning table
    let idx = idx(cube);

    // Only update the pruning table, if we've found a shorter path
    tab.data[idx] = cmp::min(tab.data[idx], depth);
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
