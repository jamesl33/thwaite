/// Rotation - TODO
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Rotation {
    /// F - TODO
    F,

    /// FP - TODO
    FP,

    /// F2 - TODO
    F2,

    /// B - TODO
    B,

    /// BP - TODO
    BP,

    /// B2 - TODO
    B2,

    /// L - TODO
    L,

    /// LP - TODO
    LP,

    /// L2 - TODO
    L2,

    /// R - TODO
    R,

    /// RP - TODO
    RP,

    /// R2 - TODO
    R2,

    /// U - TODO
    U,

    /// UP - TODO
    UP,

    /// U2 - TODO
    U2,

    /// D - TODO
    D,

    /// DP - TODO
    DP,

    /// D2 - TODO
    D2,
}

impl Rotation {
    /// face - TODO
    pub fn face(&self) -> Rotation {
        match self {
            Rotation::F | Rotation::FP | Rotation::F2 => Rotation::F,
            Rotation::B | Rotation::BP | Rotation::B2 => Rotation::B,
            Rotation::L | Rotation::LP | Rotation::L2 => Rotation::L,
            Rotation::R | Rotation::RP | Rotation::R2 => Rotation::R,
            Rotation::U | Rotation::UP | Rotation::U2 => Rotation::U,
            Rotation::D | Rotation::DP | Rotation::D2 => Rotation::D,
        }
    }
}
