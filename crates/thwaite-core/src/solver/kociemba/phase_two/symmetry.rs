//! Symmetry reduction for phase two's corner-permutation coordinate.
//!
//! Corner permutations split into orbits under the whole-cube symmetries in `SYMMETRIES`; every member of an
//! orbit shares the same pruning-table depth up to conjugation, so only the orbit's representative needs a
//! stored entry. `table::corner_edge_sym_idx` uses this to canonicalize the (much smaller) corner coordinate and
//! apply that same symmetry to the edge coordinate, giving a joint index without a 16-way search at lookup time.

use std::sync::LazyLock;

use crate::cube::{NUM_CORNERS, SYMMETRIES};
use crate::solver::maths::{idxtoperm, ptoidx};

use super::table::CORNER_PERM_STATES;

/// The number of distinct corner-permutation symmetry classes, i.e. the number of orbits `CORNER_SYM` assigns.
pub(super) static NUM_CORNER_CLASSES: LazyLock<usize> =
    LazyLock::new(|| CORNER_SYM.iter().map(|&(class, _)| class).max().unwrap() + 1);

/// For each raw corner-permutation rank (0..`CORNER_PERM_STATES`): the dense id of its symmetry orbit, and which
/// of the 16 `SYMMETRIES` maps this raw state to that orbit's chosen representative (its lowest-ranked member).
pub(super) static CORNER_SYM: LazyLock<Vec<(usize, u8)>> = LazyLock::new(|| {
    let mut table: Vec<Option<(usize, u8)>> = vec![None; CORNER_PERM_STATES];
    let mut next_class = 0;

    for raw in 0..CORNER_PERM_STATES {
        if table[raw].is_some() {
            continue;
        }

        classify_orbit(raw, &mut table, next_class);
        next_class += 1;
    }

    table.into_iter().map(|entry| entry.unwrap()).collect()
});

/// Assigns every member of `raw`'s symmetry orbit to `class` in `table`, alongside the symmetry index that maps
/// that member back to the orbit's representative.
fn classify_orbit(raw: usize, table: &mut [Option<(usize, u8)>], class: usize) {
    let perm: [u8; NUM_CORNERS] = idxtoperm(raw);

    let orbit: Vec<usize> = SYMMETRIES
        .iter()
        .map(|sym| ptoidx(&sym.conjugate_corners(&perm)))
        .collect();

    let representative = *orbit.iter().min().unwrap();

    for &member in &orbit {
        if table[member].is_some() {
            continue;
        }

        let member_perm: [u8; NUM_CORNERS] = idxtoperm(member);

        let sym_idx = SYMMETRIES
            .iter()
            .position(|sym| ptoidx(&sym.conjugate_corners(&member_perm)) == representative)
            .expect("every orbit member must map to its representative under some symmetry");

        table[member] = Some((class, sym_idx as u8));
    }
}

/// For each corner symmetry class: the raw corner-permutation rank chosen as that orbit's representative.
///
/// `SYMMETRIES[0]` is always the identity (see `build_symmetries`'s `group = vec![Symmetry::IDENTITY]` seed, which
/// is never reordered), so identity is the only symmetry that can map a raw state to itself - meaning `CORNER_SYM`
/// records `sym_idx == 0` exactly for each class's representative, and nowhere else.
pub(super) static REP_CORNER: LazyLock<Vec<usize>> = LazyLock::new(|| {
    let mut rep = vec![0; *NUM_CORNER_CLASSES];

    for (raw, &(class, sym_idx)) in CORNER_SYM.iter().enumerate() {
        if sym_idx == 0 {
            rep[class] = raw;
        }
    }

    rep
});
