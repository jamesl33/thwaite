use std::sync::LazyLock;

use crate::cube::cube::permute;
use crate::cube::{NUM_CORNERS, NUM_EDGES};
use crate::solver::maths::invert;

/// A 90 degree rotation of the whole cube about the L-R axis (U -> F -> D -> B -> U), fixing L and R as faces.
const ROT_CORNERS: [u8; NUM_CORNERS] = [5, 4, 6, 7, 0, 1, 3, 2];
const ROT_EDGES: [u8; NUM_EDGES] = [4, 5, 7, 6, 1, 0, 2, 3, 11, 8, 9, 10];

/// A 180 degree rotation of the whole cube about the F-B axis, swapping U<->D and L<->R.
const FLIP_CORNERS: [u8; NUM_CORNERS] = [2, 3, 0, 1, 6, 7, 4, 5];
const FLIP_EDGES: [u8; NUM_EDGES] = [3, 2, 1, 0, 7, 6, 5, 4, 9, 8, 11, 10];

/// The central inversion of the whole cube (an improper symmetry), swapping U<->D, F<->B and L<->R.
const MIRROR_CORNERS: [u8; NUM_CORNERS] = [6, 7, 4, 5, 2, 3, 0, 1];
const MIRROR_EDGES: [u8; NUM_EDGES] = [3, 2, 1, 0, 6, 7, 4, 5, 10, 11, 8, 9];

/// The 16 whole-cube symmetries that fix the L-R axis (as a line, not necessarily pointwise). This is the axis
/// phase two's move group `<L, R, F2, B2, U2, D2>` is built around (see `solver::kociemba::phase`'s doc comment on
/// `PHASE_TWO_VALID_MOVES` for why this codebase uses the L-R axis rather than the textbook U-D axis).
///
/// Every symmetry here is expressed purely as a corner/edge slot permutation (in the same "old[idx]/new[idx]"
/// convention as `cube::permutations`'s `PERMUTE_*` tables), since phase two's coordinates are pure permutations
/// - orientation is already fixed to identity by phase one, so no orientation-relabeling is needed.
pub static SYMMETRIES: LazyLock<[Symmetry; 16]> = LazyLock::new(build_symmetries);

/// A whole-cube symmetry, expressed as the corner and edge slot permutations it induces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symmetry {
    corners: [u8; NUM_CORNERS],
    edges: [u8; NUM_EDGES],
}

impl Symmetry {
    /// The identity symmetry: leaves every corner and edge slot unchanged.
    const IDENTITY: Symmetry = Symmetry {
        corners: [0, 1, 2, 3, 4, 5, 6, 7],
        edges: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    };

    /// Returns the symmetry that results from applying `self` after `other`.
    fn compose(&self, other: &Symmetry) -> Symmetry {
        Symmetry {
            corners: permute(self.corners, other.corners),
            edges: permute(self.edges, other.edges),
        }
    }

    /// Returns the inverse of this symmetry.
    pub fn inverse(&self) -> Symmetry {
        Symmetry {
            corners: invert(&self.corners),
            edges: invert(&self.edges),
        }
    }

    /// Returns the given corner permutation, conjugated by this symmetry.
    pub fn conjugate_corners(&self, cperms: &[u8; NUM_CORNERS]) -> [u8; NUM_CORNERS] {
        let tmp = permute(*cperms, self.inverse().corners);
        std::array::from_fn(|i| self.corners[tmp[i] as usize])
    }

    /// Returns the given edge permutation, conjugated by this symmetry.
    pub fn conjugate_edges(&self, eperms: &[u8; NUM_EDGES]) -> [u8; NUM_EDGES] {
        let tmp = permute(*eperms, self.inverse().edges);
        std::array::from_fn(|i| self.edges[tmp[i] as usize])
    }
}

/// Computes the closure of `{ROT, FLIP, MIRROR}` under composition, which should be exactly the 16-element group
/// described above. Repeatedly composes every symmetry found so far with each generator (see `new_symmetries`),
/// adding any previously-unseen results to the group, until a pass finds nothing new - a standard fixed-point
/// group-closure algorithm.
fn build_symmetries() -> [Symmetry; 16] {
    let rot = Symmetry {
        corners: ROT_CORNERS,
        edges: ROT_EDGES,
    };

    let flip = Symmetry {
        corners: FLIP_CORNERS,
        edges: FLIP_EDGES,
    };

    let mirror = Symmetry {
        corners: MIRROR_CORNERS,
        edges: MIRROR_EDGES,
    };

    let generators = [rot, flip, mirror];
    let mut group = vec![Symmetry::IDENTITY];

    loop {
        let new = new_symmetries(&group, &generators);

        if new.is_empty() {
            break;
        }

        group.extend(new);
    }

    group
        .try_into()
        .unwrap_or_else(|g: Vec<Symmetry>| panic!("expected a 16 element symmetry group, got {}", g.len()))
}

/// Composes every symmetry currently in `group` with every symmetry in `generators`, returning the distinct
/// results that aren't already members of `group` (i.e. one closure step/pass of `build_symmetries`'s algorithm).
fn new_symmetries(group: &[Symmetry], generators: &[Symmetry]) -> Vec<Symmetry> {
    let mut new = Vec::new();

    for g in group {
        for generator in generators {
            let candidate = g.compose(generator);

            if !group.contains(&candidate) && !new.contains(&candidate) {
                new.push(candidate);
            }
        }
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_has_sixteen_distinct_elements() {
        for (i, a) in SYMMETRIES.iter().enumerate() {
            for (j, b) in SYMMETRIES.iter().enumerate() {
                assert!(i == j || a != b, "symmetries {} and {} are equal", i, j);
            }
        }
    }

    #[test]
    fn group_is_closed_under_composition() {
        for a in SYMMETRIES.iter() {
            for b in SYMMETRIES.iter() {
                let composed = a.compose(b);
                assert!(
                    SYMMETRIES.iter().any(|s| *s == composed),
                    "composition of two symmetries must itself be a symmetry in the group"
                );
            }
        }
    }

    #[test]
    fn inverse_undoes_composition() {
        for a in SYMMETRIES.iter() {
            let composed = a.compose(&a.inverse());
            assert_eq!(composed, Symmetry::IDENTITY);
        }
    }

    #[test]
    fn conjugation_preserves_nonslice_slice_edge_partition() {
        // Every symmetry must map the 8 non LR-slice edge slots (0-7) amongst themselves, and the 4 LR-slice
        // edge slots (8-11) amongst themselves - phase two's combined pruning table relies on this.
        for sym in SYMMETRIES.iter() {
            let conjugated = sym.conjugate_edges(&std::array::from_fn(|i| i as u8));

            for (slot, &piece) in conjugated.iter().enumerate() {
                assert_eq!(slot >= 8, piece >= 8, "symmetry moved slice-ness of slot {}", slot);
            }
        }
    }
}
