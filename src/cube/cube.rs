use crate::cube::orientations::*;
use crate::cube::permutations::*;
use crate::cube::{Axis, Color, Corner, Edge, Rotation, CORNERS, EDGES};

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
    cperms: [u8; NUM_CORNERS],

    /// Corner orientations.
    corien: [u8; NUM_CORNERS],

    /// Edge permutations.
    eperms: [u8; NUM_EDGES],

    /// Edge orientations.
    eorien: [u8; NUM_EDGES],

    // The last move applied to the cube.
    last: Option<Rotation>,
}

impl Cube {
    /// Returns a cube, in the solved state.
    pub fn new() -> Cube {
        Cube {
            cperms: core::array::from_fn(|i| i as u8),
            corien: [0; NUM_CORNERS],
            eperms: core::array::from_fn(|i| i as u8),
            eorien: [0; NUM_EDGES],
            last: None,
        }
    }

    /// Returns a cube with the given corner/edge permutations, orientation fixed to solved and no last move
    /// recorded. Used to reconstruct a synthetic cube from a pruning-table coordinate for further move
    /// application, where only a move's permutation action matters - see
    /// `solver::kociemba::phase_two::table`'s dedicated BFS.
    pub(crate) fn from_perms(cperms: [u8; NUM_CORNERS], eperms: [u8; NUM_EDGES]) -> Cube {
        Cube {
            cperms,
            corien: [0; NUM_CORNERS],
            eperms,
            eorien: [0; NUM_EDGES],
            last: None,
        }
    }

    /// Returns the cube corner permutations.
    pub fn corner_permutations(&self) -> &[u8; NUM_CORNERS] {
        &self.cperms
    }

    /// Returns the cube corner orientations.
    pub fn corner_orientations(&self) -> &[u8; NUM_CORNERS] {
        &self.corien
    }

    /// Returns the cube edge permutations.
    pub fn edge_permutations(&self) -> &[u8; NUM_EDGES] {
        &self.eperms
    }

    /// Returns the cube edge orientations.
    pub fn edge_orientations(&self) -> &[u8; NUM_EDGES] {
        &self.eorien
    }

    /// Returns a boolean indicating whether the cube is in the solved state.
    pub fn solved(&self) -> bool {
        let eo = self.eorien == [0; NUM_EDGES];
        let ep = self.eperms == core::array::from_fn(|i| i as u8);
        let co = self.corien == [0; NUM_CORNERS];
        let cp = self.cperms == core::array::from_fn(|i| i as u8);

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
    pub fn rotate(&mut self, m: Rotation) {
        match m {
            Rotation::F => self.apply(PERMUTE_FRONT_CORNERS, Some(ORIENT_FRONT_CORNERS), PERMUTE_FRONT_EDGES, None),
            Rotation::F2 => self.apply(
                PERMUTE_FRONT_CORNERS_180,
                Some(ORIENT_FRONT_CORNERS_180),
                PERMUTE_FRONT_EDGES_180,
                None,
            ),
            Rotation::FP => self.apply(
                PERMUTE_FRONT_CORNERS_270,
                Some(ORIENT_FRONT_CORNERS_270),
                PERMUTE_FRONT_EDGES_270,
                None,
            ),
            Rotation::B => self.apply(PERMUTE_BACK_CORNERS, Some(ORIENT_BACK_CORNERS), PERMUTE_BACK_EDGES, None),
            Rotation::B2 => self.apply(
                PERMUTE_BACK_CORNERS_180,
                Some(ORIENT_BACK_CORNERS_180),
                PERMUTE_BACK_EDGES_180,
                None,
            ),
            Rotation::BP => self.apply(
                PERMUTE_BACK_CORNERS_270,
                Some(ORIENT_BACK_CORNERS_270),
                PERMUTE_BACK_EDGES_270,
                None,
            ),
            Rotation::L => self.apply(PERMUTE_LEFT_CORNERS, None, PERMUTE_LEFT_EDGES, None),
            Rotation::L2 => self.apply(PERMUTE_LEFT_CORNERS_180, None, PERMUTE_LEFT_EDGES_180, None),
            Rotation::LP => self.apply(PERMUTE_LEFT_CORNERS_270, None, PERMUTE_LEFT_EDGES_270, None),
            Rotation::R => self.apply(PERMUTE_RIGHT_CORNERS, None, PERMUTE_RIGHT_EDGES, None),
            Rotation::R2 => self.apply(PERMUTE_RIGHT_CORNERS_180, None, PERMUTE_RIGHT_EDGES_180, None),
            Rotation::RP => self.apply(PERMUTE_RIGHT_CORNERS_270, None, PERMUTE_RIGHT_EDGES_270, None),
            Rotation::U => self.apply(
                PERMUTE_UP_CORNERS,
                Some(ORIENT_UP_CORNERS),
                PERMUTE_UP_EDGES,
                Some(ORIENT_UP_EDGES),
            ),
            Rotation::U2 => self.apply(
                PERMUTE_UP_CORNERS_180,
                Some(ORIENT_UP_CORNERS_180),
                PERMUTE_UP_EDGES_180,
                Some(ORIENT_UP_EDGES_180),
            ),
            Rotation::UP => self.apply(
                PERMUTE_UP_CORNERS_270,
                Some(ORIENT_UP_CORNERS_270),
                PERMUTE_UP_EDGES_270,
                Some(ORIENT_UP_EDGES_270),
            ),
            Rotation::D => self.apply(
                PERMUTE_DOWN_CORNERS,
                Some(ORIENT_DOWN_CORNERS),
                PERMUTE_DOWN_EDGES,
                Some(ORIENT_DOWN_EDGES),
            ),
            Rotation::D2 => self.apply(
                PERMUTE_DOWN_CORNERS_180,
                Some(ORIENT_DOWN_CORNERS_180),
                PERMUTE_DOWN_EDGES_180,
                Some(ORIENT_DOWN_EDGES_180),
            ),
            Rotation::DP => self.apply(
                PERMUTE_DOWN_CORNERS_270,
                Some(ORIENT_DOWN_CORNERS_270),
                PERMUTE_DOWN_EDGES_270,
                Some(ORIENT_DOWN_EDGES_270),
            ),
        };

        self.last = Some(m);
    }

    /// Returns the last move applied to the cube, if any.
    pub fn last(&self) -> Option<Rotation> {
        self.last
    }

    /// Returns a boolean indicating whether the given move is redundant, based on the last move applied to the cube.
    pub fn redundant(&self, m: &Rotation) -> bool {
        if let Some(last) = self.last {
            return last.face() == m.face() || last.face() == m.opposite();
        }

        false
    }

    /// Applies a single move's corner/edge permutation, and (where the move twists pieces) orientation delta.
    /// Shared by every `Rotation` arm in `rotate()` - each 90/180/270 degree turn differs only in which
    /// precomputed tables it passes here.
    fn apply(
        &mut self,
        pc: [u8; NUM_CORNERS],
        oc: Option<[i8; NUM_CORNERS]>,
        pe: [u8; NUM_EDGES],
        oe: Option<[i8; NUM_EDGES]>,
    ) {
        self.cperms = permute(self.cperms, pc);

        if let Some(oc) = oc {
            self.corien = orient(self.corien, self.cperms, oc, CORNER_ORIENTATIONS);
        }

        self.eperms = permute(self.eperms, pe);

        if let Some(oe) = oe {
            self.eorien = orient(self.eorien, self.eperms, oe, EDGE_ORIENTATIONS);
        }
    }
}

impl From<&str> for Cube {
    /// Parses cube-state from a "cube string" (e.g. Y...R...B...W...O...G...).
    fn from(cs: &str) -> Self {
        // Convert the string into a vector
        let cs = Vec::from(cs);

        // A cube string is 6 faces of 9 facelets each.
        assert_eq!(
            cs.len(),
            54,
            "cube string must be exactly 54 characters, got {}",
            cs.len()
        );

        // Convert that into a vector of colors
        let cs: Vec<Color> = cs.iter().map(|c| c.into()).collect();

        // Convert that into a vector of the underlying faces
        let cs: Vec<Vec<Color>> = cs.chunks(9).map(|s| s.into()).collect();

        // Defer parsing to another implementation of 'from'
        Cube::from(cs)
    }
}

impl From<Vec<Vec<Color>>> for Cube {
    /// Parses cube-state from a vector of faces, and their colors; the order is up, right, front, down, left and back.
    fn from(cs: Vec<Vec<Color>>) -> Self {
        // Create a new solved cube
        let mut cube = Cube::new();

        // Populate the edge permutations and orientations
        edges_from_cs(&mut cube, &cs);

        // Populate the corner permutations and orientations
        corners_from_cs(&mut cube, &cs);

        cube
    }
}

/// Populates the given cubes edge state, using the provided vectorized cube-string representation.
fn edges_from_cs(cube: &mut Cube, cs: &Vec<Vec<Color>>) {
    // ed is the desired location for the edge (i.e. the solved position)
    for (idx, ed) in EDGES.iter().enumerate() {
        // ea is the edge that's actually in that position
        let ea = Edge::new(cs[ed.a.face as usize][ed.a.idx], cs[ed.b.face as usize][ed.b.idx]);

        // The identifier for the edge
        let id = ea.id();

        // Populate the permutation (i.e. edge ea is in the position ed)
        cube.eperms[idx] = id as u8;

        // Grab the solved color for this face; it doesn't matter which, providing the following code using the same
        let want = ea.a.face.color();

        // Grab the color that's actually in that position
        let facelet = if cs[ed.a.face as usize][ed.a.idx] == want {
            ed.a
        } else {
            ed.b
        };

        // Check if they're both "blue", to determine if the orientation is "good" or "bad"
        //
        // https://stackoverflow.com/a/63640220
        if ea.a.blue() == facelet.blue() {
            continue;
        }

        // Mark the orientation as bad, as we're moving from blue to yellow or vice versa
        cube.eorien[id] = 1;
    }
}

/// Populates the given cubes corner orientation, using the provided vectorized cube-string representation.
fn corners_from_cs(cube: &mut Cube, cs: &Vec<Vec<Color>>) {
    // cd is the desired location for the corner (i.e. the solved position)
    for (idx, cd) in CORNERS.iter().enumerate() {
        // ca is the corner that's actually in that position
        let ca = Corner::new(
            cs[cd.a.face as usize][cd.a.idx],
            cs[cd.b.face as usize][cd.b.idx],
            cs[cd.c.face as usize][cd.c.idx],
        );

        // Get the id of the desired corner
        let cd_id = cd.id();

        // Get the id of the actual corner
        let ca_id = ca.id();

        // Populate the permutation (i.e. corner ca is in the position cd)
        cube.cperms[idx] = ca_id as u8;

        // Get the faces in the x, y, z order
        let axis = cd.axis();

        // Extract the x axis
        let x = axis[Axis::X as usize];

        // Extract the y axis
        let y = axis[Axis::Y as usize];

        // Extract the z axis
        let z = axis[Axis::Z as usize];

        // Construct a vector of the actual colors in each axis
        let colors = vec![
            cs[x.face as usize][x.idx],
            cs[y.face as usize][y.idx],
            cs[z.face as usize][z.idx],
        ];

        // Rotate those colors clockwise
        let mut colors_cw = colors.clone();
        colors_cw.rotate_right(2);

        // Rotate those colors anti-clockwise
        let mut colors_acw = colors.clone();
        colors_acw.rotate_right(1);

        // Due to the geometry of the cube, we have to swap the rotations for half the corners
        if cd_id >= 4 {
            std::mem::swap(&mut colors_cw, &mut colors_acw);
        }

        // Returns a boolean indicating whether the orange or red face is in position x (i.e. is "good")
        let or = |colors: Vec<Color>| {
            return colors[Axis::X as usize] == Color::Orange || colors[Axis::X as usize] == Color::Red;
        };

        // The corner is good as it's touching the nearest left/right color
        if or(colors) {
            cube.corien[ca_id] = 0;
        }

        // The corner is twisted clockwise relative to the nearest left/right color
        if or(colors_cw) {
            cube.corien[ca_id] = 1;
        }

        // The corner is twisted anti-clockwise relative to the nearest left/right color
        if or(colors_acw) {
            cube.corien[ca_id] = 2;
        }
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

/// Permutes the given pieces using the provided rotation definition.
pub(crate) fn permute<const N: usize>(src: [u8; N], rot: [u8; N]) -> [u8; N] {
    let mut cop: [u8; N] = src;

    for (i, v) in rot.iter().enumerate() {
        cop[i] = src[*v as usize];
    }

    // The summation of the pieces should not have changed, they should have just been permuted
    debug_assert_eq!(
        cop.iter().map(|&x| x as usize).sum::<usize>(),
        (0..N).sum::<usize>()
    );

    cop
}

/// Orients the given pieces using the provided rotation definition.
fn orient<const N: usize>(orien: [u8; N], perms: [u8; N], rot: [i8; N], rem: usize) -> [u8; N] {
    let mut cop: [u8; N] = orien;

    for (i, v) in rot.iter().enumerate() {
        let idx = perms[i] as usize;
        cop[idx] = (cop[idx] as isize + *v as isize).rem_euclid(rem as isize) as u8;
    }

    // The remainder of the summation of piece orientations should equal zero
    debug_assert_eq!(cop.iter().map(|&x| x as usize).sum::<usize>().rem_euclid(rem), 0);

    cop
}
