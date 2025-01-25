use crate::cube::{Cube, Rotation};

/// Group - TODO
#[derive(Debug)]
pub enum Group {
    Zero,
    One,
    Two,
    Three,
}

impl Group {
    /// from - TODO
    pub fn from(cube: &Cube) -> Group {
        if cube.edge_orientations().iter().sum::<isize>() > 0 {
            return Group::Zero;
        }

        unimplemented!();
    }

    /// moves - TODO
    ///
    /// TODO (jamesl33) Add support for rejecting duplicate moves.
    pub fn moves(&self) -> Vec<Rotation> {
        match self {
            Group::Zero => vec![
                Rotation::F,
                Rotation::F2,
                Rotation::FP,
                Rotation::B,
                Rotation::B2,
                Rotation::BP,
                Rotation::L,
                Rotation::L2,
                Rotation::LP,
                Rotation::R,
                Rotation::R2,
                Rotation::RP,
                Rotation::U,
                Rotation::U2,
                Rotation::UP,
                Rotation::D,
                Rotation::D2,
                Rotation::DP,
            ],
            Group::One => vec![
                Rotation::F,
                Rotation::F2,
                Rotation::FP,
                Rotation::B,
                Rotation::B2,
                Rotation::BP,
                Rotation::L,
                Rotation::L2,
                Rotation::LP,
                Rotation::R,
                Rotation::R2,
                Rotation::RP,
                Rotation::U2,
                Rotation::D2,
            ],
            Group::Two => vec![
                Rotation::F2,
                Rotation::B2,
                Rotation::L,
                Rotation::L2,
                Rotation::LP,
                Rotation::R,
                Rotation::R2,
                Rotation::RP,
                Rotation::U2,
                Rotation::D2,
            ],
            Group::Three => vec![
                Rotation::F2,
                Rotation::B2,
                Rotation::L2,
                Rotation::R2,
                Rotation::U2,
                Rotation::D2,
            ],
        }
    }
}
