use std::cmp;

use serde::{Deserialize, Serialize};

use crate::cube::Cube;
use crate::solver::group::Group;
use crate::solver::maths::factorial;

/// The size of the G3 pruning table, which is a one dimensional array.
const SIZE: usize = 663552;

/// The pruning table for the G3.
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// The underlying data, where each index represents a cube state and its depth from the solved state.
    data: Vec<usize>,
}

impl Table {
    /// Calculates and returns a new G3 pruning table.
    pub fn new() -> Table {
        g3()
    }

    /// Returns the number of moves the given cube is, from being in G3.
    pub fn depth(&self, cube: &Cube) -> usize {
        self.data[idx(cube)]
    }
}

/// Creates a new pattern database for G3.
fn g3() -> Table {
    // As documented the max depth from G3 is fifteen.
    //
    // http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf
    const DEPTH: usize = 15;

    // We initialize the pruning table at the max depth, and search for the cheaper distances
    let mut tab: Table = Table {
        data: vec![DEPTH; SIZE],
    };

    // Which has a distance of zero
    tab.data[0] = 0;

    // We start searching from a solved cube
    let start: Cube = Cube::new();

    // Perform a depth first search, applying all the valid G3 moves and recording the depth from the solved state
    start.search(Group::Three.moves(), DEPTH - 1, &mut |cube, depth| {
        // Calculate the index in the pruning table
        let idx = idx(cube);

        // Only update the pruning table, if we've found a shorter path
        tab.data[idx] = cmp::min(tab.data[idx], depth);
    });

    tab
}

/// Returns the index within the pruning table for the given cube.
///
/// https://github.com/itaysadeh/rubiks-cube-solver/blob/b80fd03698055bd630067358c6175a240ecd64b1/Solver/Thistlethwaite/Databases/G3_G4_database.cpp#L3-L56.
fn idx(cube: &Cube) -> usize {
    // Edge permutations
    let eperms = cube.edge_permutations();

    // Corner permutations
    let cperms = cube.corner_permutations();

    // E-slice edges
    let mut e = *array_ref![eperms, 4, 2];

    for i in 0..2 {
        e[i] &= 3;
    }

    // M-slice edges
    let mut m = *array_ref![eperms, 8, 4];

    for i in 0..4 {
        m[i] &= 3;
    }

    // S-slice edges
    let mut s = *array_ref![eperms, 0, 4];

    // Set the indices of the edge pieces to a value between 0 and 3
    for i in 0..4 {
        s[i] &= 3;
    }

    // Corners which are currently occupying which position in its tetrad
    let mut c = *array_ref![cperms, 4, 4];

    // Set the indices of the corners pieces to a value between 0 and 3
    for i in 0..4 {
        c[i] &= 3;
    }

    // M-slice rank
    let mr = ptoidx(&m);

    // S-slice rank
    let sr = ptoidx(&s);

    // E-slice rank
    let er = ctoidx::<4, 2>(&e);

    // Edges index
    let eidx = mr * 288 + sr * 12 + er;

    // Corner rank
    let cr = cperms[0];

    // Corner index
    let cidx = ptoidx(&c) * 4 + (cr & 3);

    // The combined E-C index
    eidx * 96 + cidx
}

/// Returns the index as determined by the combination of the pieces positions.
///
/// https://github.com/itaysadeh/rubiks-cube-solver/blob/b80fd03698055bd630067358c6175a240ecd64b1/Util/indexer.h#L95-L120.
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

/// Returns the number of combinations, for the given selection `k` from `n`.
fn comb(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    if n == k {
        return 1;
    }

    factorial(n) / factorial(n - k)
}

/// Returns the index for the given piece permutations.
///
/// https://github.com/itaysadeh/rubiks-cube-solver/blob/b80fd03698055bd630067358c6175a240ecd64b1/Util/indexer.h#L52-L77.
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

        i += 1;

        j -= 1;
    }

    idx
}
