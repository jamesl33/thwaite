use crate::cube::orientations::*;
use crate::cube::permutations::*;
use crate::cube::Rotation;

/// The number of corners on a valid 3x3 cube.
pub const NUM_CORNERS: usize = 8;

/// The number of edges on a valid 3x3 cube.
pub const NUM_EDGES: usize = 12;

/// Models a 3x3 Rubik's cube, exposing functionality required to produce pruning tables and search for solutions.
///
/// TODO (jamesl33): Move this to a module.
#[derive(Debug, Clone, Copy)]
pub struct Cube {
    /// Corner permutations.
    cperms: [usize; NUM_CORNERS],

    /// Corner orientations.
    corien: [usize; NUM_CORNERS],

    /// Edge permutations.
    eperms: [usize; NUM_EDGES],

    /// Edge orientations.
    eorien: [usize; NUM_EDGES],

    // The last move applied to the cube.
    last: Option<Rotation>,
}

impl Cube {
    /// Returns a cube, in the solved state.
    pub fn new() -> Cube {
        Cube {
            cperms: core::array::from_fn(|i| i),
            corien: [0; NUM_CORNERS],
            eperms: core::array::from_fn(|i| i),
            eorien: [0; NUM_EDGES],
            last: None,
        }
    }

    /// Returns the cube corner permutations.
    pub fn corner_permutations(&self) -> &[usize; NUM_CORNERS] {
        &self.cperms
    }

    /// Returns the cube corner orientations.
    pub fn corner_orientations(&self) -> &[usize; NUM_CORNERS] {
        &self.corien
    }

    /// Returns the cube edge permutations.
    pub fn edge_permutations(&self) -> &[usize; NUM_EDGES] {
        &self.eperms
    }

    /// Returns the cube edge orientations.
    pub fn edge_orientations(&self) -> &[usize; NUM_EDGES] {
        &self.eorien
    }

    /// Returns a boolean indicating whether the cube is in the solved state.
    pub fn solved(&self) -> bool {
        let eo = self.eorien == [0; NUM_EDGES];
        let ep = self.eperms == core::array::from_fn(|i| i);
        let co = self.corien == [0; NUM_CORNERS];
        let cp = self.cperms == core::array::from_fn(|i| i);

        eo && ep && co && cp
    }

    /// Performs a depth first search applying the given moves, until a limit is reached; runs the given callback for
    /// each cube state visited.
    pub fn search<F>(&self, moves: &[Rotation], limit: usize, func: &mut F)
    where
        F: FnMut(&Cube, usize),
    {
        dfs(*self, moves, 1, limit, func);
    }

    /// Applies the given rotations to the cube.
    ///
    /// TODO (jamesl33): The 180 degree turns can optimized into a single operation.
    /// TODO (jamesl33): The 90 degree prime turns can optimized into a single operation.
    pub fn rotate(&mut self, m: Rotation) {
        match m {
            Rotation::F => rotate(self, &mut Self::rotate_front, 1),
            Rotation::F2 => rotate(self, &mut Self::rotate_front, 2),
            Rotation::FP => rotate(self, &mut Self::rotate_front, 3),
            Rotation::B => rotate(self, &mut Self::rotate_back, 1),
            Rotation::B2 => rotate(self, &mut Self::rotate_back, 2),
            Rotation::BP => rotate(self, &mut Self::rotate_back, 3),
            Rotation::L => rotate(self, &mut Self::rotate_left, 1),
            Rotation::L2 => rotate(self, &mut Self::rotate_left, 2),
            Rotation::LP => rotate(self, &mut Self::rotate_left, 3),
            Rotation::R => rotate(self, &mut Self::rotate_right, 1),
            Rotation::R2 => rotate(self, &mut Self::rotate_right, 2),
            Rotation::RP => rotate(self, &mut Self::rotate_right, 3),
            Rotation::U => rotate(self, &mut Self::rotate_up, 1),
            Rotation::U2 => rotate(self, &mut Self::rotate_up, 2),
            Rotation::UP => rotate(self, &mut Self::rotate_up, 3),
            Rotation::D => rotate(self, &mut Self::rotate_down, 1),
            Rotation::D2 => rotate(self, &mut Self::rotate_down, 2),
            Rotation::DP => rotate(self, &mut Self::rotate_down, 3),
        };

        self.last = Some(m);
    }

    /// Returns a boolean indicating whether the given move is redundant, based on the last move applied to the cube.
    pub fn redundant(&self, m: &Rotation) -> bool {
        if let Some(last) = self.last {
            return last.face() == m.face() || last.face() == m.opposite();
        }

        false
    }

    /// Rotates the front face clockwise by 90 degrees.
    fn rotate_front(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_FRONT_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_FRONT_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_FRONT_EDGES);
        // We omit orienting the front edges, as it's a no-op.
    }

    /// Rotates the back face clockwise by 90 degrees.
    fn rotate_back(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_BACK_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_BACK_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_BACK_EDGES);
        // We omit orienting the back edges, as it's a no-op.
    }

    /// Rotates the left face clockwise by 90 degrees.
    fn rotate_left(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_LEFT_CORNERS);
        self.eperms = permute(self.eperms, PERMUTE_LEFT_EDGES);
        // No orientation required
    }

    /// Rotates the right face clockwise by 90 degrees.
    fn rotate_right(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_RIGHT_CORNERS);
        self.eperms = permute(self.eperms, PERMUTE_RIGHT_EDGES);
        // No orientation required
    }

    /// Rotates the up face clockwise by 90 degrees.
    fn rotate_up(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_UP_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_UP_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_UP_EDGES);
        self.eorien = orient(self.eorien, self.eperms, ORIENT_UP_EDGES, EDGE_ORIENTATIONS);
    }

    /// Rotates the down face clockwise by 90 degrees.
    fn rotate_down(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_DOWN_CORNERS);
        self.corien = orient(self.corien, self.cperms, ORIENT_DOWN_CORNERS, CORNER_ORIENTATIONS);
        self.eperms = permute(self.eperms, PERMUTE_DOWN_EDGES);
        self.eorien = orient(self.eorien, self.eperms, ORIENT_DOWN_EDGES, EDGE_ORIENTATIONS);
    }
}

/// Performs a depth first search applying the given moves, until a limit is reached; runs the given callback for each
/// cube state visited.
fn dfs<F>(cube: Cube, moves: &[Rotation], depth: usize, limit: usize, func: &mut F)
where
    F: FnMut(&Cube, usize),
{
    for mv in moves {
        if cube.redundant(mv) {
            continue;
        }

        let mut cube = cube;

        cube.rotate(*mv);

        func(&cube, depth);

        // We've reached out limit, stop searching
        if depth >= limit {
            continue;
        }

        dfs(cube, moves, depth + 1, limit, func);
    }
}

/// Runs the given rotation `n` number of times.
fn rotate<F>(cube: &mut Cube, func: &mut F, n: usize)
where
    F: FnMut(&mut Cube),
{
    for _ in 0..n {
        func(cube)
    }
}

/// Permutes the given pieces using the provided rotation definition.
fn permute<const N: usize>(src: [usize; N], rot: [usize; N]) -> [usize; N] {
    let mut cop: [usize; N] = src;

    for (i, v) in rot.iter().enumerate() {
        cop[i] = src[*v];
    }

    // The summation of the pieces should not have changed, they should have just been permuted
    debug_assert_eq!(cop.iter().sum::<usize>(), (0..N).sum::<usize>());

    cop
}

/// Orients the given pieces using the provided rotation definition.
fn orient<const N: usize>(orien: [usize; N], perms: [usize; N], rot: [isize; N], rem: usize) -> [usize; N] {
    let mut cop: [usize; N] = orien;

    for (i, v) in rot.iter().enumerate() {
        cop[perms[i]] = (cop[perms[i]] as isize + *v).rem_euclid(rem as isize) as usize;
    }

    // The remainder of the summation of piece orientations should equal zero
    debug_assert_eq!(cop.iter().sum::<usize>().rem_euclid(rem), 0);

    cop
}
