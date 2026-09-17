use enum_utils::IterVariants;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IteratorRandom;
use rand::Rng;

/// Represents a rotation (move) that may be applied to the Rubik's Cube.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, IterVariants)]
pub enum Rotation {
    /// A clockwise 90 degree twist of the front face.
    F,

    /// A counter-clockwise 90 degree twist of the front face.
    FP,

    /// A 180 degree twist of the front face.
    F2,

    /// A clockwise 90 degree twist of the back face.
    B,

    /// A counter-clockwise 90 degree twist of the back face.
    BP,

    /// A 180 degree twist of the back face.
    B2,

    /// A clockwise 90 degree twist of the left face.
    L,

    /// A counter-clockwise 90 degree twist of the left face.
    LP,

    /// A 180 degree twist of the left face.
    L2,

    /// A clockwise 90 degree twist of the right face.
    R,

    /// A counter-clockwise 90 degree twist of the right face.
    RP,

    /// A 180 degree twist of the right face.
    R2,

    /// A clockwise 90 degree twist of the up face.
    U,

    /// A counter-clockwise 90 degree twist of the up face.
    UP,

    /// A 180 degree twist of the up face.
    U2,

    /// A clockwise 90 degree twist of the down face.
    D,

    /// A counter-clockwise 90 degree twist of the down face.
    DP,

    /// A 180 degree twist of the down face.
    D2,
}

impl Rotation {
    /// Returns the face being turned (e.g. front, back).
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

    /// Returns opposite of the face being turned (e.g. front, back).
    pub fn opposite(&self) -> Rotation {
        match self {
            Rotation::F | Rotation::FP | Rotation::F2 => Rotation::B,
            Rotation::B | Rotation::BP | Rotation::B2 => Rotation::F,
            Rotation::L | Rotation::LP | Rotation::L2 => Rotation::R,
            Rotation::R | Rotation::RP | Rotation::R2 => Rotation::L,
            Rotation::U | Rotation::UP | Rotation::U2 => Rotation::D,
            Rotation::D | Rotation::DP | Rotation::D2 => Rotation::U,
        }
    }
}

impl Distribution<Rotation> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        Rotation::iter().choose(rng).unwrap()
    }
}
