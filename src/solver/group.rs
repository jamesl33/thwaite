use crate::cube::{Cube, Rotation, NUM_CORNERS};

/// GROUP_ZERO_VALID_MOVES - TODO
const GROUP_ZERO_VALID_MOVES: [Rotation; 18] = [
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
];

/// GROUP_ONE_VALID_MOVES - TODO
const GROUP_ONE_VALID_MOVES: [Rotation; 14] = [
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
];

/// GROUP_TWO_VALID_MOVES - TODO
const GROUP_TWO_VALID_MOVES: [Rotation; 10] = [
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
];

/// GROUP_THREE_VALID_MOVES - TODO
const GROUP_THREE_VALID_MOVES: [Rotation; 6] = [
    Rotation::F2,
    Rotation::B2,
    Rotation::L2,
    Rotation::R2,
    Rotation::U2,
    Rotation::D2,
];

/// Group - TODO
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Group {
    Zero,
    One,
    Two,
    Three,
}

impl Group {
    /// moves - TODO
    pub fn moves(&self) -> &[Rotation] {
        match self {
            Group::Zero => &GROUP_ZERO_VALID_MOVES,
            Group::One => &GROUP_ONE_VALID_MOVES,
            Group::Two => &GROUP_TWO_VALID_MOVES,
            Group::Three => &GROUP_THREE_VALID_MOVES,
        }
    }
}
