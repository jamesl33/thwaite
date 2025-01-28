use std::cmp;

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;

/// FACTORIAL - TODO
const FACTORIALS: [usize; 13] = factorials();

/// N_SIZE - TODO
const N_SIZE: usize = combinations(NUM_EDGES, 4);

/// M_SIZE - TODO
const M_SIZE: usize = usize::pow(3, 7);

/// SIZE - TODO
const SIZE: usize = N_SIZE * M_SIZE;

/// Table - TODO
#[derive(Debug)]
pub struct Table {
    /// data - TODO
    data: Vec<usize>,
}

impl Table {
    /// new - TODO
    pub fn new() -> Table {
        g1()
    }
}

/// g1 - TODO
fn g1() -> Table {
    /// DEPTH - TODO
    const DEPTH: usize = 10;

    // // We initialize the pruning table at the max depth, and search for the cheaper distances
    let mut tab = Table {
        data: vec![DEPTH; SIZE],
    };

    // Which has a distance of zero
    tab.data[0] = 0;

    // TODO (jamesl33): This is the wrong starting state.
    let start: Cube = Cube::new();

    // TODO
    start.search(Group::One.moves(), DEPTH - 1, &mut |cube, depth| {
        // TODO
        let oidx = otoidx(cube.corner_orientations());

        // TODO
        let pidx = ptoidx(cube.edge_permutations());

        // TODO
        let idx = oidx * N_SIZE + pidx;

        // Only update the pruning table, if we've found a shorter path
        tab.data[idx] = cmp::min(tab.data[idx], depth);
    });

    tab
}

/// otoidx - TODO
fn otoidx<const N: usize>(orien: &[usize; N]) -> usize {
    let mut idx: usize = 0;

    for i in 0..N - 1 {
        idx = idx * 3 + orien[i]
    }

    // TODO
    debug_assert!(idx < M_SIZE);

    idx
}

/// ptoidx - TODO
///
/// https://www.jaapsch.net/puzzles/compindx.htm
fn ptoidx(perms: &[usize; NUM_EDGES]) -> usize {
    let mut t = 0;
    let mut r = 4;

    for i in (0..NUM_EDGES).rev() {
        // We only care about edges 8-12
        if perms[i] <= 7 {
            continue;
        }

        // TODO
        t += combinations(i, r);

        // TODO
        r -= 1;
    }

    // TODO
    debug_assert!(t < N_SIZE);

    t
}

/// comb - TODO
const fn combinations(i: usize, r: usize) -> usize {
    if i < r {
        return 0;
    }

    if i == r {
        return 1;
    }

    FACTORIALS[i] / (FACTORIALS[r] * FACTORIALS[i - r])
}

/// factorials - TODO
const fn factorials<const N: usize>() -> [usize; N] {
    let mut factorials = [0; N];
    let mut i = 0;

    loop {
        if i == N {
            break
        }

        // TODO
        factorials[i] = factorial(i);

        // TODO
        i += 1;
    }

    factorials
}

/// factorial - TODO
const fn factorial(n: usize) -> usize {
    // TODO
    if n <= 1 {
        return 1;
    }

    let mut a = 1;
    let mut i = 1;

    loop {
        if i == n {
            break
        }

        // TODO
        a *= i;

        // TODO
        i += 1;
    }

    return a;
}
