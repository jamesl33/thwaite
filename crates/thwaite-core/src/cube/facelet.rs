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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::{Column, Row};

    #[test]
    fn new_computes_grid_index_from_row_and_column() {
        assert_eq!(Facelet::new(Face::Up, Row::Up, Column::Left).idx, 0);
        assert_eq!(Facelet::new(Face::Up, Row::Up, Column::Right).idx, 2);
        assert_eq!(Facelet::new(Face::Up, Row::Middle, Column::Middle).idx, 4);
        assert_eq!(Facelet::new(Face::Up, Row::Down, Column::Right).idx, 8);
    }

    #[test]
    fn front_and_back_facelets_are_never_blue() {
        for face in [Face::Front, Face::Back] {
            for idx in 0..9 {
                assert!(!(Facelet { face, idx }).blue(), "face {face:?} idx {idx}");
            }
        }
    }

    #[test]
    fn left_and_right_facelets_are_always_blue() {
        for face in [Face::Left, Face::Right] {
            for idx in 0..9 {
                assert!((Facelet { face, idx }).blue(), "face {face:?} idx {idx}");
            }
        }
    }

    #[test]
    fn up_and_down_facelets_are_blue_except_at_the_middle_left_and_right() {
        let middle_left = Row::Middle as usize + Column::Left as usize;
        let middle_right = Row::Middle as usize + Column::Right as usize;

        for face in [Face::Up, Face::Down] {
            for idx in 0..9 {
                let expect_blue = idx != middle_left && idx != middle_right;

                assert_eq!((Facelet { face, idx }).blue(), expect_blue, "face {face:?} idx {idx}");
            }
        }
    }
}
