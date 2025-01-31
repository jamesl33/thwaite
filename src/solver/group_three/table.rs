use std::{array, cmp};

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;
use crate::solver::maths::factorial;

/// SIZE - TODO
const SIZE: usize = 663552;

/// Table - TODO
#[derive(Debug)]
pub struct Table {
    /// data - TODO
    data: Vec<usize>,
}

impl Table {
    /// new - TODO
    pub fn new() -> Table {
        g0()
    }
}

/// g0 - TODO
fn g0() -> Table {
    /// DEPTH - TODO
    const DEPTH: usize = 12;

    // We initialize the pruning table at the max depth, and search for the cheaper distances
    let mut tab: Table = Table {
        data: vec![DEPTH; SIZE],
    };

    // Which has a distance of zero
    tab.data[0] = 0;

    // We start searching from a solved cube
    let start: Cube = Cube::new();

    // TODO
    start.search(Group::Three.moves(), DEPTH - 1, &mut |cube, depth| {
        // TODO
        let idx = idx(cube);

        // Only update the pruning table, if we've found a shorter path
        tab.data[idx] = cmp::min(tab.data[idx], depth);
    });

    tab
}

/// idx - TODO
fn idx(cube: &Cube) -> usize {
    // TODO
    let eperms = cube.edge_permutations();

    // TODO
    let cperms = cube.corner_permutations();

    // TODO
    let mut e = *array_ref![eperms, 4, 4];

    // TODO
    for i in 0..4 {
        e[i] &= 3;
    }

    // TODO
    let mut m: [usize; 4] = *array_ref![eperms, 8, 4];

    // TODO
    for i in 0..4 {
        m[i] &= 3;
    }

    // TODO
    let s: [usize; 4] = *array_ref![eperms, 0, 4];

    // TODO
    let c: [usize; 4] = *array_ref![cperms, 0, 4];

    // TODO
    let cr = cube.corner_permutations()[4];

    // TODO
    let sr = ctoidx::<4, 2>(&lehmer(&s));

    // TODO
    let cidx = ptoidx(&lehmer(&c)) * 4 + (cr >> 1);

    // TODO
    let eidx = ptoidx(&lehmer(&m)) * 288 + ptoidx(&lehmer(&e)) * 12 + sr;

    // TODO
    eidx * 96 + cidx
}

/// lehmer - TODO
fn lehmer<const N: usize>(perms: &[usize; N]) -> [usize; N] {
    let mut lehmer = *perms;

    for i in 1..N {
        for j in (0..=i).rev() {
            if perms[j] >= perms[i] {
                continue;
            }

            lehmer[i] -= 1;
        }
    }

    return lehmer;
}

/// ptoidx - TODO
fn ptoidx<const N: usize>(lehmer: &[usize; N]) -> usize {
    let mut idx = 0;

    let mut i = 0;
    let mut j = N - 1;

    while i < N {
        idx += lehmer[i] * factorial(j);
        i += 1;
        j -= 1;
    }

    idx
}

/// ctoidx - TODO
fn ctoidx<const N: usize, const K: usize>(lehmer: &[usize; N]) -> usize {
    let mut idx = 0;

    for i in 0..N {
        idx += lehmer[i] * comb(N - 1 - i, K - 1 - i);
    }

    idx
}

/// comb - TODO
fn comb(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    if n == k {
        return 1;
    }

    return factorial(n) / factorial(n - k);
}
