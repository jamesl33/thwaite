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
        g3()
    }

    /// depth - TODO
    pub fn depth(&self, cube: &Cube) -> usize {
        self.data[idx(cube)]
    }
}

/// g3 - TODO
fn g3() -> Table {
    /// DEPTH - TODO
    const DEPTH: usize = 10;

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
    let mut e = *array_ref![eperms, 4, 2];

    // TODO
    for i in 0..2 {
        e[i] &= 3;
    }

    // TODO
    let mut m = *array_ref![eperms, 8, 4];

    // TODO
    for i in 0..4 {
        m[i] &= 3;
    }

    // TODO
    let mut s = *array_ref![eperms, 0, 4];

    // TODO
    for i in 0..4 {
        s[i] &= 3;
    }

    // TODO
    let mut c = *array_ref![cperms, 4, 4];

    // TODO
    for i in 0..4 {
        c[i] &= 3;
    }

    // TODO
    let mr = ptoidx(&m);

    // TODO
    let sr = ptoidx(&s);

    // TODO
    let er = ctoidx::<4, 2>(&e);

    // TODO
    let eidx = mr * 288 + sr * 12 + er;

    // TODO
    let cr = cperms[0];

    // TODO
    let cidx = ptoidx(&c) * 4 + (cr & 3);

    // TODO
    eidx * 96 + cidx
}

/// ctoidx - TODO
fn ctoidx<const N: usize, const K: usize>(perms: &[usize; K]) -> usize {
    let mut lehmer = *perms;

    for i in 1..K {
        for j in (0..i).rev() {
            if perms[j] < perms[i] {
                lehmer[i] -= 1;
            }
        }
    }

    let mut idx = 0;

    for i in 0..K {
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

    factorial(n) / factorial(n - k)
}

/// ptoidx - TODO
fn ptoidx<const N: usize>(perms: &[usize; N]) -> usize {
    let mut lehmer = *perms;

    for i in 1..N {
        for j in (0..i).rev() {
            if perms[j] < perms[i] {
                lehmer[i] -= 1;
            }
        }
    }

    let mut idx = 0;

    let mut i = 0;
    let mut j = N - 1;

    while i < N && j > 0 {
        idx += lehmer[i] * factorial(j);

        // TODO
        i += 1;

        // TODO
        j -= 1;
    }

    idx
}
