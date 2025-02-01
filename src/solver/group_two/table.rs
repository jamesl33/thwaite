use std::cmp;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cube::{Cube, NUM_CORNERS, NUM_EDGES};
use crate::solver::group::Group;
use crate::solver::maths::{combinations, factorial};

/// INITIAL - TODO
const INITIAL: usize = 96;

/// N_SIZE - TODO
const N_SIZE: usize = factorial(8);

/// M_SIZE - TODO
const M_SIZE: usize = combinations(8, 4);

/// SIZE - TODO
const SIZE: usize = N_SIZE * M_SIZE;

/// Table - TODO
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// data - TODO
    data: Vec<usize>,
}

impl Table {
    /// new - TODO
    pub fn new() -> Table {
        g2()
    }

    /// depth - TODO
    pub fn depth(&self, cube: &Cube) -> usize {
        self.data[idx(cube)]
    }
}

/// g2 - TODO
///
/// TODO (jamesl33): This search could be done in parallel.
fn g2() -> Table {
    /// DEPTH - TODO
    ///
    /// TODO (jamesl33): This depth isn't enough, but DFS is slow without visited branch pruning (i.e. 96 cubes).
    const DEPTH: usize = 8;

    // TODO
    let mut tab: Table = Table {
        data: vec![DEPTH; SIZE],
    };

    // TODO
    let cubes = initial();

    // TODO
    for cube in cubes {
        dfs(&mut tab, cube, DEPTH - 1)
    }

    tab
}

/// dfs - TODO
fn dfs(tab: &mut Table, cube: Cube, limit: usize) {
    // TODO
    update(tab, &cube, 0);

    // TODO
    cube.search(Group::Two.moves(), limit, &mut |cube, depth| update(tab, cube, depth));
}

/// update - TODO
fn update(tab: &mut Table, cube: &Cube, depth: usize) {
    // TODO
    let idx = idx(cube);

    // Only update the pruning table, if we've found a shorter path
    tab.data[idx] = cmp::min(tab.data[idx], depth);
}

/// idx - TODO
fn idx(cube: &Cube) -> usize {
    // TODO
    let cpidx = ptoidx(cube.corner_permutations());

    // TODO
    let epidx = pctoidx::<8>(cube.edge_permutations()[..NUM_EDGES - 4].try_into().unwrap());

    // TODO
    cpidx * M_SIZE + epidx
}

/// initial - TODO
fn initial() -> Vec<Cube> {
    /// DEPTH - TODO
    const DEPTH: usize = 5;

    // TODO
    let mut cubes = HashMap::<usize, Cube>::new();

    // We start searching from a solved cube
    let start: Cube = Cube::new();

    // TODO
    start.search(Group::Two.moves(), DEPTH - 1, &mut |cube, _| {
        // TODO
        if !cino(cube.corner_permutations()) {
            return;
        }

        // TODO
        let idx = ptoidx(cube.corner_permutations());

        if cubes.contains_key(&idx) {
            return;
        }

        // TODO
        cubes.insert(idx, *cube);
    });

    cubes.into_values().collect()
}

/// cino - TODO
fn cino(cperms: &[usize; NUM_CORNERS]) -> bool {
    for i in 0..4 {
        if cperms[i] > 3 {
            return false;
        }
    }

    true
}

/// ptoidx - TODO
///
/// https://www.jaapsch.net/puzzles/compindx.htm#perm
pub fn ptoidx<const N: usize>(perms: &[usize; N]) -> usize {
    let mut t = 0;

    for i in 0..N - 1 {
        t = t * (N - i);

        for j in i + 1..N {
            if perms[i] > perms[j] {
                t += 1;
            }
        }
    }

    t
}

/// pctoidx - TODO
///
/// https://www.jaapsch.net/puzzles/compindx.htm#comb
fn pctoidx<const N: usize>(perms: &[usize; N]) -> usize {
    let mut t = 0;
    let mut r = 4;

    for i in (0..N).rev() {
        // TODO
        if perms[i] <= 3 {
            continue;
        }

        // TODO
        t += combinations(i, r);

        // TODO
        r -= 1;
    }

    // TODO
    debug_assert!(t < M_SIZE);

    t
}
