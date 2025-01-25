use crate::cube;
use crate::cube::orientations::*;
use crate::cube::permutations::*;

/// NUM_CORNERS - TODO
pub const NUM_CORNERS: usize = 8;

/// NUM_EDGES - TODO
pub const NUM_EDGES: usize = 12;

/// Cube - TODO
///
/// TODO (jamesl33): Move this to a module.
#[derive(Debug)]
pub struct Cube {
    /// cperms - TODO
    cperms: [isize; NUM_CORNERS],

    /// corien - TODO
    corien: [isize; NUM_CORNERS],

    /// eperms - TODO
    eperms: [isize; NUM_EDGES],

    /// eorien - TODO
    eorien: [isize; NUM_EDGES],
}

impl Cube {
    /// new - TODO
    pub fn new() -> Cube {
        Cube {
            cperms: [0, 1, 2, 3, 4, 5, 6, 7],
            corien: [0, 0, 0, 0, 0, 0, 0, 0],
            eperms: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            eorien: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    /// rotate - TODO
    pub fn rotate(&mut self, m: cube::Rotation) {
        match m {
            cube::Rotation::Front => self.rotate_front(),
            cube::Rotation::Back => self.rotate_back(),
            cube::Rotation::Left => self.rotate_left(),
            cube::Rotation::Right => self.rotate_right(),
            cube::Rotation::Up => self.rotate_up(),
            cube::Rotation::Down => self.rotate_down(),
        }
    }

    /// rotate_front - TODO
    fn rotate_front(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_FRONT_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_FRONT_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_FRONT_EDGES);
        // We omit orienting the front edges, as it's a no-op.
    }

    /// rotate_back - TODO
    fn rotate_back(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_BACK_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_BACK_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_BACK_EDGES);
        // We omit orienting the back edges, as it's a no-op.
    }

    /// rotate_left - TODO
    fn rotate_left(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_LEFT_CORNERS);
        self.eperms = permute(self.eperms, PERMUTE_LEFT_EDGES);
    }

    /// rotate_right - TODO
    fn rotate_right(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_RIGHT_CORNERS);
        self.eperms = permute(self.eperms, PERMUTE_RIGHT_EDGES);
    }

    /// rotate_up - TODO
    fn rotate_up(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_UP_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_UP_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_UP_EDGES);
        self.eorien = orient(self.eorien, self.eperms, ORIENT_UP_EDGES, EDGE_ORIENTATIONS);
    }

    /// rotate_down - TODO
    fn rotate_down(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_DOWN_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_DOWN_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_DOWN_EDGES);
        self.eorien = orient(self.eorien, self.eperms, ORIENT_DOWN_EDGES, EDGE_ORIENTATIONS);
    }
}

/// permute - TODO
fn permute<const N: usize>(src: [isize; N], rot: [usize; N]) -> [isize; N] {
    let mut cop: [isize; N] = src;

    for (i, v) in rot.iter().enumerate() {
        cop[i] = src[*v];
    }

    debug_assert_eq!(cop.iter().sum::<isize>(), (0..N).sum::<usize>() as isize);

    cop
}

/// orient - TODO
fn orient<const N: usize>(orien: [isize; N], perms: [isize; N], rot: [isize; N], rem: isize) -> [isize; N] {
    let mut cop: [isize; N] = orien;

    for (i, v) in rot.iter().enumerate() {
        cop[perms[i] as usize] = (cop[perms[i] as usize] + *v).rem_euclid(rem);
    }

    debug_assert_eq!(cop.iter().sum::<isize>().rem_euclid(rem), 0);

    cop
}
