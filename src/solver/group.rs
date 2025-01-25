use crate::cube::Rotation;

/// Group - TODO
pub enum Group {
    One,
    Two,
    Three,
    Four,
}

impl Group {
    /// moves - TODO
    ///
    /// TODO (jamesl33) Add support for rejecting duplicate moves.
    pub fn moves(self) -> Vec<Rotation> {
        match self {
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
                Rotation::U,
                Rotation::U2,
                Rotation::UP,
                Rotation::D,
                Rotation::D2,
                Rotation::DP,
            ],
            Group::Two=> vec![
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
            Group::Three=> vec![
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
            Group::Four=> vec![
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
