use crate::cube::{Column, Face, Row};

/// Represents a "facelet" on the cube (i.e. a single colored face).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Facelet {
    /// The face of the cube this facelet is on.
    pub face: Face,

    /// The index into the vector of colors where 0 is the top left, 3 is the middle left and 6 is the bottom left.
    pub idx: usize,
}

impl Facelet {
    /// Returns a new facelet.
    pub const fn new(face: Face, row: Row, column: Column) -> Self {
        Facelet {
            face: face,
            idx: row as usize + column as usize,
        }
    }

    /// Returns whether this facelet is blue (where false means it's yellow).
    ///
    /// https://stackoverflow.com/a/63640220
    pub fn blue(self: &Self) -> bool {
        match self.face {
            // The front/back face edges are all yellow
            Face::Front | Face::Back => false,
            // The right/left face edges are all blue
            Face::Right | Face::Left => true,
            // The up/down faces, it depends on whether the edge is on the left/right.
            Face::Up | Face::Down => {
                // The edge is on the middle left
                let ml = self.idx == Row::Middle as usize + Column::Left as usize;

                // The edge is on the middle right
                let mr = self.idx == Row::Middle as usize + Column::Right as usize;

                // If it's on the middle left or right, it's yellow
                return !(ml || mr);
            }
        }
    }
}
