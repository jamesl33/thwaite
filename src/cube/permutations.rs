use crate::{NUM_CORNERS, NUM_EDGES};

/// PERMUTE_FRONT_CORNERS - TODO
pub const PERMUTE_FRONT_CORNERS: [usize; NUM_CORNERS] = [0, 6, 2, 4, 1, 5, 3, 7];

/// PERMUTE_FRONT_EDGES - TODO
pub const PERMUTE_FRONT_EDGES: [usize; NUM_EDGES] = [0, 1, 2, 3, 4, 9, 8, 7, 5, 6, 10, 11];

/// PERMUTE_BACK_CORNERS - TODO
///
/// TODO (jamesl33): Note that this is wrong in the paper.
pub const PERMUTE_BACK_CORNERS: [usize; NUM_CORNERS] = [7, 1, 5, 3, 4, 0, 6, 2];

/// PERMUTE_BACK_EDGES - TODO
pub const PERMUTE_BACK_EDGES: [usize; NUM_EDGES] = [0, 1, 2, 3, 11, 5, 6, 10, 8, 9, 4, 7];

/// PERMUTE_LEFT_CORNERS - TODO
pub const PERMUTE_LEFT_CORNERS: [usize; NUM_CORNERS] = [5, 4, 2, 3, 0, 1, 6, 7];

/// PERMUTE_LEFT_EDGES - TODO
pub const PERMUTE_LEFT_EDGES: [usize; NUM_EDGES] = [4, 5, 2, 3, 1, 0, 6, 7, 8, 9, 10, 11];

/// PERMUTE_RIGHT_CORNERS - TODO
pub const PERMUTE_RIGHT_CORNERS: [usize; NUM_CORNERS] = [0, 1, 7, 6, 4, 5, 2, 3];

/// PERMUTE_RIGHT_EDGES - TODO
pub const PERMUTE_RIGHT_EDGES: [usize; NUM_EDGES] = [0, 1, 6, 7, 4, 5, 3, 2, 8, 9, 10, 11];

/// PERMUTE_UP_CORNERS - TODO
pub const PERMUTE_UP_CORNERS: [usize; NUM_CORNERS] = [4, 1, 2, 7, 3, 5, 6, 0];

/// PERMUTE_UP_EDGES - TODO
pub const PERMUTE_UP_EDGES: [usize; NUM_EDGES] = [8, 1, 11, 3, 4, 5, 6, 7, 2, 9, 10, 0];

/// PERMUTE_DOWN_CORNERS - TODO
pub const PERMUTE_DOWN_CORNERS: [usize; NUM_CORNERS] = [0, 5, 6, 3, 4, 2, 1, 7];

/// ORIENT_DOWN_EDGES - TODO
pub const ORIENT_DOWN_EDGES: [isize; NUM_EDGES] = [0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0];
