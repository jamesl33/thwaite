use crate::cube::{NUM_CORNERS, NUM_EDGES};

/// The number of orientations for a corner piece.
pub const CORNER_ORIENTATIONS: usize = 3;

/// The number of orientations for an edge piece.
pub const EDGE_ORIENTATIONS: usize = 2;

/// The rotation matrix for the front-face corners where rotation = old + new % orientations.
pub const ORIENT_FRONT_CORNERS: [isize; NUM_CORNERS] = [0, 1, 0, 1, -1, 0, -1, 0];

/// The rotation matrix for the back-face corners where rotation = old + new % orientations.
pub const ORIENT_BACK_CORNERS: [isize; NUM_CORNERS] = [1, 0, 1, 0, 0, -1, 0, -1];

/// The rotation matrix for the up-face corners where rotation = old + new % orientations.
pub const ORIENT_UP_CORNERS: [isize; NUM_CORNERS] = [-1, 0, 0, -1, 1, 0, 0, 1];

/// The rotation matrix for the up-face edges where rotation = old + new % orientations.
pub const ORIENT_UP_EDGES: [isize; NUM_EDGES] = [1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1];

/// The rotation matrix for the down-face corners where rotation = old + new % orientations.
pub const ORIENT_DOWN_CORNERS: [isize; NUM_CORNERS] = [0, -1, -1, 0, 0, 1, 1, 0];

/// The rotation matrix for the down-face edges where rotation = old + new % orientations.
pub const PERMUTE_DOWN_EDGES: [usize; NUM_EDGES] = [0, 10, 2, 9, 4, 5, 6, 7, 8, 1, 3, 11];
