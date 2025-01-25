use crate::cube;

/// CORNER_ORIENTATIONS - TODO
pub const CORNER_ORIENTATIONS: isize = 3;

/// EDGE_ORIENTATIONS - TODO
pub const EDGE_ORIENTATIONS: isize = 2;

/// ORIENT_FRONT_CORNERS - TODO
pub const ORIENT_FRONT_CORNERS: [isize; cube::NUM_CORNERS] = [0, 1, 0, 1, -1, 0, -1, 0];

/// ORIENT_BACK_CORNERS - TODO
pub const ORIENT_BACK_CORNERS: [isize; cube::NUM_CORNERS] = [1, 0, 1, 0, 0, -1, 0, -1];

/// ORIENT_UP_CORNERS - TODO
pub const ORIENT_UP_CORNERS: [isize; cube::NUM_CORNERS] = [-1, 0, 0, -1, 1, 0, 0, 1];

/// ORIENT_UP_EDGES - TODO
pub const ORIENT_UP_EDGES: [isize; cube::NUM_EDGES] = [1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1];

/// ORIENT_DOWN_CORNERS - TODO
pub const ORIENT_DOWN_CORNERS: [isize; cube::NUM_CORNERS] = [0, -1, -1, 0, 0, 1, 1, 0];

/// PERMUTE_DOWN_EDGES - TODO
pub const PERMUTE_DOWN_EDGES: [usize; cube::NUM_EDGES] = [0, 10, 2, 9, 4, 5, 6, 7, 8, 1, 3, 11];
