use crate::cube::Rotation;

/// The valid moves when searching for phase one, i.e. all eighteen moves.
pub const PHASE_ONE_VALID_MOVES: [Rotation; 18] = [
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

/// The valid moves when searching for phase two.
///
/// NOTE: The textbook Kociemba definition of phase two's group is `<U, D, L2, R2, F2, B2>`, i.e. it assumes
/// quarter turns of U/D preserve orientation. In this codebase's convention orientation is tracked relative to
/// the L/R axis instead - `Cube::rotate_left`/`rotate_right` never touch orientation at all, `rotate_front`/
/// `rotate_back` only disturb corner orientation, and `rotate_up`/`rotate_down` disturb both - so the group of
/// moves that preserve both corner and edge orientation here is `<L, R, F2, B2, U2, D2>` instead.
pub const PHASE_TWO_VALID_MOVES: [Rotation; 10] = [
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
