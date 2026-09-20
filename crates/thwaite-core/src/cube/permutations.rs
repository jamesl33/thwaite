use crate::{NUM_CORNERS, NUM_EDGES};

/// The rotation matrix for the front-face corners; where old[idx] and new[idx] are swapped.
pub const PERMUTE_FRONT_CORNERS: [u8; NUM_CORNERS] = [0, 6, 2, 4, 1, 5, 3, 7];

/// The rotation matrix for the front-face edges; where old[idx] and new[idx] are swapped.
pub const PERMUTE_FRONT_EDGES: [u8; NUM_EDGES] = [0, 1, 2, 3, 4, 9, 8, 7, 5, 6, 10, 11];

/// The rotation matrix for the back-face corner; where old[idx] and new[idx] are swapped.
///
/// NOTE: This rotation is documented incorrectly in http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf.
pub const PERMUTE_BACK_CORNERS: [u8; NUM_CORNERS] = [7, 1, 5, 3, 4, 0, 6, 2];

/// The rotation matrix for the back-face edges; where old[idx] and new[idx] are swapped.
pub const PERMUTE_BACK_EDGES: [u8; NUM_EDGES] = [0, 1, 2, 3, 11, 5, 6, 10, 8, 9, 4, 7];

/// The rotation matrix for the left-face corners; where old[idx] and new[idx] are swapped.
pub const PERMUTE_LEFT_CORNERS: [u8; NUM_CORNERS] = [5, 4, 2, 3, 0, 1, 6, 7];

/// The rotation matrix for the left-face edges; where old[idx] and new[idx] are swapped.
pub const PERMUTE_LEFT_EDGES: [u8; NUM_EDGES] = [4, 5, 2, 3, 1, 0, 6, 7, 8, 9, 10, 11];

/// The rotation matrix for the right-face corners; where old[idx] and new[idx] are swapped.
pub const PERMUTE_RIGHT_CORNERS: [u8; NUM_CORNERS] = [0, 1, 7, 6, 4, 5, 2, 3];

/// The rotation matrix for the right-face edges; where old[idx] and new[idx] are swapped.
pub const PERMUTE_RIGHT_EDGES: [u8; NUM_EDGES] = [0, 1, 6, 7, 4, 5, 3, 2, 8, 9, 10, 11];

/// The rotation matrix for the up-face corners; where old[idx] and new[idx] are swapped.
pub const PERMUTE_UP_CORNERS: [u8; NUM_CORNERS] = [4, 1, 2, 7, 3, 5, 6, 0];

/// The rotation matrix for the up-face edges; where old[idx] and new[idx] are swapped.
pub const PERMUTE_UP_EDGES: [u8; NUM_EDGES] = [8, 1, 11, 3, 4, 5, 6, 7, 2, 9, 10, 0];

/// The rotation matrix for the down-face corners; where old[idx] and new[idx] are swapped.
pub const PERMUTE_DOWN_CORNERS: [u8; NUM_CORNERS] = [0, 5, 6, 3, 4, 2, 1, 7];

/// The rotation matrix for the down-face edges where rotation = old + new % orientations.
pub const PERMUTE_DOWN_EDGES: [u8; NUM_EDGES] = [0, 10, 2, 9, 4, 5, 6, 7, 8, 1, 3, 11];

/// The rotation matrix for a 180 degree turn of the front-face corners.
pub const PERMUTE_FRONT_CORNERS_180: [u8; NUM_CORNERS] = compose(PERMUTE_FRONT_CORNERS, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the front-face corners.
pub const PERMUTE_FRONT_CORNERS_270: [u8; NUM_CORNERS] = compose(PERMUTE_FRONT_CORNERS, 3);

/// The rotation matrix for a 180 degree turn of the front-face edges.
pub const PERMUTE_FRONT_EDGES_180: [u8; NUM_EDGES] = compose(PERMUTE_FRONT_EDGES, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the front-face edges.
pub const PERMUTE_FRONT_EDGES_270: [u8; NUM_EDGES] = compose(PERMUTE_FRONT_EDGES, 3);

/// The rotation matrix for a 180 degree turn of the back-face corners.
pub const PERMUTE_BACK_CORNERS_180: [u8; NUM_CORNERS] = compose(PERMUTE_BACK_CORNERS, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the back-face corners.
pub const PERMUTE_BACK_CORNERS_270: [u8; NUM_CORNERS] = compose(PERMUTE_BACK_CORNERS, 3);

/// The rotation matrix for a 180 degree turn of the back-face edges.
pub const PERMUTE_BACK_EDGES_180: [u8; NUM_EDGES] = compose(PERMUTE_BACK_EDGES, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the back-face edges.
pub const PERMUTE_BACK_EDGES_270: [u8; NUM_EDGES] = compose(PERMUTE_BACK_EDGES, 3);

/// The rotation matrix for a 180 degree turn of the left-face corners.
pub const PERMUTE_LEFT_CORNERS_180: [u8; NUM_CORNERS] = compose(PERMUTE_LEFT_CORNERS, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the left-face corners.
pub const PERMUTE_LEFT_CORNERS_270: [u8; NUM_CORNERS] = compose(PERMUTE_LEFT_CORNERS, 3);

/// The rotation matrix for a 180 degree turn of the left-face edges.
pub const PERMUTE_LEFT_EDGES_180: [u8; NUM_EDGES] = compose(PERMUTE_LEFT_EDGES, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the left-face edges.
pub const PERMUTE_LEFT_EDGES_270: [u8; NUM_EDGES] = compose(PERMUTE_LEFT_EDGES, 3);

/// The rotation matrix for a 180 degree turn of the right-face corners.
pub const PERMUTE_RIGHT_CORNERS_180: [u8; NUM_CORNERS] = compose(PERMUTE_RIGHT_CORNERS, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the right-face corners.
pub const PERMUTE_RIGHT_CORNERS_270: [u8; NUM_CORNERS] = compose(PERMUTE_RIGHT_CORNERS, 3);

/// The rotation matrix for a 180 degree turn of the right-face edges.
pub const PERMUTE_RIGHT_EDGES_180: [u8; NUM_EDGES] = compose(PERMUTE_RIGHT_EDGES, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the right-face edges.
pub const PERMUTE_RIGHT_EDGES_270: [u8; NUM_EDGES] = compose(PERMUTE_RIGHT_EDGES, 3);

/// The rotation matrix for a 180 degree turn of the up-face corners.
pub const PERMUTE_UP_CORNERS_180: [u8; NUM_CORNERS] = compose(PERMUTE_UP_CORNERS, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the up-face corners.
pub const PERMUTE_UP_CORNERS_270: [u8; NUM_CORNERS] = compose(PERMUTE_UP_CORNERS, 3);

/// The rotation matrix for a 180 degree turn of the up-face edges.
pub const PERMUTE_UP_EDGES_180: [u8; NUM_EDGES] = compose(PERMUTE_UP_EDGES, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the up-face edges.
pub const PERMUTE_UP_EDGES_270: [u8; NUM_EDGES] = compose(PERMUTE_UP_EDGES, 3);

/// The rotation matrix for a 180 degree turn of the down-face corners.
pub const PERMUTE_DOWN_CORNERS_180: [u8; NUM_CORNERS] = compose(PERMUTE_DOWN_CORNERS, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the down-face corners.
pub const PERMUTE_DOWN_CORNERS_270: [u8; NUM_CORNERS] = compose(PERMUTE_DOWN_CORNERS, 3);

/// The rotation matrix for a 180 degree turn of the down-face edges.
pub const PERMUTE_DOWN_EDGES_180: [u8; NUM_EDGES] = compose(PERMUTE_DOWN_EDGES, 2);

/// The rotation matrix for a 270 degree (counter-clockwise 90 degree) turn of the down-face edges.
pub const PERMUTE_DOWN_EDGES_270: [u8; NUM_EDGES] = compose(PERMUTE_DOWN_EDGES, 3);

/// Composes the given single 90 degree turn's permutation with itself `times` times, producing the permutation
/// equivalent to applying that 90 degree turn `times` times in a row. Used below to derive each face's 180/270
/// degree tables from its 90 degree one at compile time, rather than looping `Cube::rotate_*` 2-3 times per move
/// at runtime.
const fn compose<const N: usize>(rot: [u8; N], times: usize) -> [u8; N] {
    let mut out = [0u8; N];
    let mut i = 0;

    while i < N {
        let mut idx = i;
        let mut t = 0;

        while t < times {
            idx = rot[idx] as usize;
            t += 1;
        }

        out[i] = idx as u8;
        i += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_permutation<const N: usize>(p: &[u8; N]) -> bool {
        let mut seen = [false; N];

        for &v in p {
            if v as usize >= N || seen[v as usize] {
                return false;
            }

            seen[v as usize] = true;
        }

        true
    }

    #[test]
    fn is_permutation_rejects_a_repeated_value() {
        assert!(!is_permutation(&[0, 0, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn every_base_permutation_is_a_valid_bijection() {
        for p in [
            PERMUTE_FRONT_CORNERS,
            PERMUTE_BACK_CORNERS,
            PERMUTE_LEFT_CORNERS,
            PERMUTE_RIGHT_CORNERS,
            PERMUTE_UP_CORNERS,
            PERMUTE_DOWN_CORNERS,
        ] {
            assert!(is_permutation(&p), "{p:?} is not a valid permutation");
        }

        for p in [
            PERMUTE_FRONT_EDGES,
            PERMUTE_BACK_EDGES,
            PERMUTE_LEFT_EDGES,
            PERMUTE_RIGHT_EDGES,
            PERMUTE_UP_EDGES,
            PERMUTE_DOWN_EDGES,
        ] {
            assert!(is_permutation(&p), "{p:?} is not a valid permutation");
        }
    }

    /// A physical quarter turn, applied four times, returns every piece to where it started - regardless of the
    /// (arbitrary) convention `permute()` uses to read these tables.
    #[test]
    fn every_base_quarter_turn_has_order_four() {
        for p in [
            PERMUTE_FRONT_CORNERS,
            PERMUTE_BACK_CORNERS,
            PERMUTE_LEFT_CORNERS,
            PERMUTE_RIGHT_CORNERS,
            PERMUTE_UP_CORNERS,
            PERMUTE_DOWN_CORNERS,
        ] {
            assert_eq!(compose(p, 4), [0, 1, 2, 3, 4, 5, 6, 7], "{p:?} does not have order 4");
        }

        for p in [
            PERMUTE_FRONT_EDGES,
            PERMUTE_BACK_EDGES,
            PERMUTE_LEFT_EDGES,
            PERMUTE_RIGHT_EDGES,
            PERMUTE_UP_EDGES,
            PERMUTE_DOWN_EDGES,
        ] {
            assert_eq!(
                compose(p, 4),
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                "{p:?} does not have order 4"
            );
        }
    }

    #[test]
    fn composing_a_single_application_is_a_no_op() {
        assert_eq!(compose(PERMUTE_FRONT_CORNERS, 1), PERMUTE_FRONT_CORNERS);
    }
}
