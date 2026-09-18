use crate::cube::permutations::{
    PERMUTE_BACK_CORNERS, PERMUTE_DOWN_CORNERS, PERMUTE_DOWN_EDGES, PERMUTE_FRONT_CORNERS, PERMUTE_UP_CORNERS,
    PERMUTE_UP_EDGES,
};
use crate::cube::{NUM_CORNERS, NUM_EDGES};

/// The number of orientations for a corner piece.
pub const CORNER_ORIENTATIONS: usize = 3;

/// The number of orientations for an edge piece.
pub const EDGE_ORIENTATIONS: usize = 2;

/// The rotation matrix for the front-face corners where rotation = old + new % orientations.
pub const ORIENT_FRONT_CORNERS: [i8; NUM_CORNERS] = [0, 1, 0, 1, -1, 0, -1, 0];

/// The rotation matrix for the back-face corners where rotation = old + new % orientations.
pub const ORIENT_BACK_CORNERS: [i8; NUM_CORNERS] = [1, 0, 1, 0, 0, -1, 0, -1];

/// The rotation matrix for the up-face corners where rotation = old + new % orientations.
pub const ORIENT_UP_CORNERS: [i8; NUM_CORNERS] = [-1, 0, 0, -1, 1, 0, 0, 1];

/// The rotation matrix for the up-face edges where rotation = old + new % orientations.
pub const ORIENT_UP_EDGES: [i8; NUM_EDGES] = [1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1];

/// The rotation matrix for the down-face corners where rotation = old + new % orientations.
pub const ORIENT_DOWN_CORNERS: [i8; NUM_CORNERS] = [0, -1, -1, 0, 0, 1, 1, 0];

/// The rotation matrix for the down-face edges; where old[idx] and new[idx] are swapped.
pub const ORIENT_DOWN_EDGES: [i8; NUM_EDGES] = [0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0];

/// The orientation delta for a 180 degree turn of the front-face corners.
pub const ORIENT_FRONT_CORNERS_180: [i8; NUM_CORNERS] = compose(PERMUTE_FRONT_CORNERS, ORIENT_FRONT_CORNERS, 2);

/// The orientation delta for a 270 degree (counter-clockwise 90 degree) turn of the front-face corners.
pub const ORIENT_FRONT_CORNERS_270: [i8; NUM_CORNERS] = compose(PERMUTE_FRONT_CORNERS, ORIENT_FRONT_CORNERS, 3);

/// The orientation delta for a 180 degree turn of the back-face corners.
pub const ORIENT_BACK_CORNERS_180: [i8; NUM_CORNERS] = compose(PERMUTE_BACK_CORNERS, ORIENT_BACK_CORNERS, 2);

/// The orientation delta for a 270 degree (counter-clockwise 90 degree) turn of the back-face corners.
pub const ORIENT_BACK_CORNERS_270: [i8; NUM_CORNERS] = compose(PERMUTE_BACK_CORNERS, ORIENT_BACK_CORNERS, 3);

/// The orientation delta for a 180 degree turn of the up-face corners.
pub const ORIENT_UP_CORNERS_180: [i8; NUM_CORNERS] = compose(PERMUTE_UP_CORNERS, ORIENT_UP_CORNERS, 2);

/// The orientation delta for a 270 degree (counter-clockwise 90 degree) turn of the up-face corners.
pub const ORIENT_UP_CORNERS_270: [i8; NUM_CORNERS] = compose(PERMUTE_UP_CORNERS, ORIENT_UP_CORNERS, 3);

/// The orientation delta for a 180 degree turn of the up-face edges.
pub const ORIENT_UP_EDGES_180: [i8; NUM_EDGES] = compose(PERMUTE_UP_EDGES, ORIENT_UP_EDGES, 2);

/// The orientation delta for a 270 degree (counter-clockwise 90 degree) turn of the up-face edges.
pub const ORIENT_UP_EDGES_270: [i8; NUM_EDGES] = compose(PERMUTE_UP_EDGES, ORIENT_UP_EDGES, 3);

/// The orientation delta for a 180 degree turn of the down-face corners.
pub const ORIENT_DOWN_CORNERS_180: [i8; NUM_CORNERS] = compose(PERMUTE_DOWN_CORNERS, ORIENT_DOWN_CORNERS, 2);

/// The orientation delta for a 270 degree (counter-clockwise 90 degree) turn of the down-face corners.
pub const ORIENT_DOWN_CORNERS_270: [i8; NUM_CORNERS] = compose(PERMUTE_DOWN_CORNERS, ORIENT_DOWN_CORNERS, 3);

/// The orientation delta for a 180 degree turn of the down-face edges.
pub const ORIENT_DOWN_EDGES_180: [i8; NUM_EDGES] = compose(PERMUTE_DOWN_EDGES, ORIENT_DOWN_EDGES, 2);

/// The orientation delta for a 270 degree (counter-clockwise 90 degree) turn of the down-face edges.
pub const ORIENT_DOWN_EDGES_270: [i8; NUM_EDGES] = compose(PERMUTE_DOWN_EDGES, ORIENT_DOWN_EDGES, 3);

/// Composes a single 90 degree turn's orientation delta with itself `times` times, producing the delta equivalent
/// to applying that turn `times` times in a row. `perm_rot` must be that same turn's paired permutation - see
/// `permutations::compose` for the paired permutation composition.
const fn compose<const N: usize>(perm_rot: [u8; N], orient_rot: [i8; N], times: usize) -> [i8; N] {
    let mut out = [0i8; N];
    let mut i = 0;

    while i < N {
        let mut idx = i;
        let mut sum: i8 = 0;
        let mut t = 0;

        // The piece ending up at position `i` after `times` applications passed through positions
        // `i, perm_rot(i), perm_rot(perm_rot(i)), ...` (read in reverse chronological order), picking up delta
        // `orient_rot` at each position it was in; this sums that orbit. Summing (rather than reducing modulo the
        // orientation count after each step, as `Cube::orient` does at runtime) is equivalent, since `Cube::orient`'s
        // `rem_euclid` after each individual application composes into a single `rem_euclid` of the total.
        while t < times {
            sum += orient_rot[idx];
            idx = perm_rot[idx] as usize;
            t += 1;
        }

        out[i] = sum;
        i += 1;
    }

    out
}
