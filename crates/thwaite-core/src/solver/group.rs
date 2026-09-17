use crate::cube::Rotation;

/// The valid moves when in group-zero; all the moves, except slices and cube-rotations.
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

/// The valid moves when in group-one; removal of single turns of the U/D faces.
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

/// The valid moves when in group-two, removal of almost all the single face turns.
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

/// The valid moves when in group-three; only 180 degree turns of all the faces.
const GROUP_THREE_VALID_MOVES: [Rotation; 6] = [
    Rotation::F2,
    Rotation::B2,
    Rotation::L2,
    Rotation::R2,
    Rotation::U2,
    Rotation::D2,
];

/// A Thistlewate group where moves are progressively removed once certain criteria are met, to reduce the search space.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Group {
    Zero,
    One,
    Two,
    Three,
}

impl Group {
    /// Returns the valid moves for the group.
    pub fn moves(&self) -> &[Rotation] {
        match self {
            Group::Zero => &GROUP_ZERO_VALID_MOVES,
            Group::One => &GROUP_ONE_VALID_MOVES,
            Group::Two => &GROUP_TWO_VALID_MOVES,
            Group::Three => &GROUP_THREE_VALID_MOVES,
        }
    }
}
