use crate::cube::orientations::*;
use crate::cube::permutations::*;
use crate::cube::Rotation;

/// NUM_CORNERS - TODO
pub const NUM_CORNERS: usize = 8;

/// NUM_EDGES - TODO
pub const NUM_EDGES: usize = 12;

/// Cube - TODO
///
/// TODO (jamesl33): Move this to a module.
#[derive(Debug, Clone, Copy)]
pub struct Cube {
    /// cperms - TODO
    cperms: [usize; NUM_CORNERS],

    /// corien - TODO
    corien: [usize; NUM_CORNERS],

    /// eperms - TODO
    eperms: [usize; NUM_EDGES],

    /// eorien - TODO
    eorien: [usize; NUM_EDGES],

    // last - TODO
    last: Option<Rotation>,
}

impl Cube {
    /// new - TODO
    pub fn new() -> Cube {
        Cube {
            cperms: [0, 1, 2, 3, 4, 5, 6, 7],
            corien: [0, 0, 0, 0, 0, 0, 0, 0],
            eperms: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            eorien: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            last: None,
        }
    }

    /// corner_permutations - TODO
    pub fn corner_permutations(&self) -> &[usize; NUM_CORNERS] {
        &self.cperms
    }

    /// corner_orientations - TODO
    pub fn corner_orientations(&self) -> &[usize; NUM_CORNERS] {
        &self.corien
    }

    /// edge_permutations - TODO
    pub fn edge_permutations(&self) -> &[usize; NUM_EDGES] {
        &self.eperms
    }

    /// edge_orientations - TODO
    pub fn edge_orientations(&self) -> &[usize; NUM_EDGES] {
        &self.eorien
    }

    /// solved - TODO
    pub fn solved(&self) -> bool {
        let eo = self.eorien == [0; NUM_EDGES];
        let ep = monotonic(&self.eperms);
        let co = self.corien == [0; NUM_CORNERS];
        let cp = monotonic(&self.cperms);

        eo && ep && co && cp
    }

    /// search - TODO
    pub fn search<F>(&self, moves: &[Rotation], limit: usize, func: &mut F)
    where
        F: FnMut(&Cube, usize),
    {
        dfs(*self, moves, 1, limit, func);
    }

    /// rotate - TODO
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

    /// redundant - TODO
    pub fn redundant(&self, m: &Rotation) -> bool {
        if let Some(last) = self.last {
            return last.face() == m.face() || last.face() == m.opposite();
        }

        false
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
        // No orientation required
    }

    /// rotate_right - TODO
    fn rotate_right(&mut self) {
        self.cperms = permute(self.cperms, PERMUTE_RIGHT_CORNERS);
        self.eperms = permute(self.eperms, PERMUTE_RIGHT_EDGES);
        // No orientation required
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

/// dfs - TODO
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

/// rotate - TODO
fn rotate<F>(cube: &mut Cube, func: &mut F, n: usize)
where
    F: FnMut(&mut Cube),
{
    for _ in 0..n {
        func(cube)
    }
}

/// permute - TODO
fn permute<const N: usize>(src: [usize; N], rot: [usize; N]) -> [usize; N] {
    let mut cop: [usize; N] = src;

    for (i, v) in rot.iter().enumerate() {
        cop[i] = src[*v];
    }

    debug_assert_eq!(cop.iter().sum::<usize>(), (0..N).sum::<usize>());

    cop
}

/// orient - TODO
fn orient<const N: usize>(orien: [usize; N], perms: [usize; N], rot: [isize; N], rem: usize) -> [usize; N] {
    let mut cop: [usize; N] = orien;

    for (i, v) in rot.iter().enumerate() {
        cop[perms[i]] = (cop[perms[i]] as isize + *v).rem_euclid(rem as isize) as usize;
    }

    debug_assert_eq!(cop.iter().sum::<usize>().rem_euclid(rem), 0);

    cop
}

/// monotonic - TODO
fn monotonic<const N: usize>(a: &[usize; N]) -> bool {
    for i in 0..N {
        if a[i] != i {
            return false;
        }
    }

    true
}
