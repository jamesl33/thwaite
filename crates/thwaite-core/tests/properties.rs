//! Property tests for `Cube`'s move algebra: rotations should behave like a group, regardless of which state
//! they're applied from.

use proptest::prelude::*;
use thwaite_core::cube::{Cube, Rotation};

/// A strategy producing any single face rotation.
fn rotation() -> impl Strategy<Value = Rotation> {
    prop_oneof![
        Just(Rotation::F),
        Just(Rotation::FP),
        Just(Rotation::F2),
        Just(Rotation::B),
        Just(Rotation::BP),
        Just(Rotation::B2),
        Just(Rotation::L),
        Just(Rotation::LP),
        Just(Rotation::L2),
        Just(Rotation::R),
        Just(Rotation::RP),
        Just(Rotation::R2),
        Just(Rotation::U),
        Just(Rotation::UP),
        Just(Rotation::U2),
        Just(Rotation::D),
        Just(Rotation::DP),
        Just(Rotation::D2),
    ]
}

/// A strategy producing any quarter turn (90 degree) rotation.
fn quarter_turn() -> impl Strategy<Value = Rotation> {
    prop_oneof![
        Just(Rotation::F),
        Just(Rotation::FP),
        Just(Rotation::B),
        Just(Rotation::BP),
        Just(Rotation::L),
        Just(Rotation::LP),
        Just(Rotation::R),
        Just(Rotation::RP),
        Just(Rotation::U),
        Just(Rotation::UP),
        Just(Rotation::D),
        Just(Rotation::DP),
    ]
}

/// A strategy producing any half turn (180 degree) rotation.
fn half_turn() -> impl Strategy<Value = Rotation> {
    prop_oneof![
        Just(Rotation::F2),
        Just(Rotation::B2),
        Just(Rotation::L2),
        Just(Rotation::R2),
        Just(Rotation::U2),
        Just(Rotation::D2),
    ]
}

/// A strategy producing a sequence of rotations, used to reach an arbitrary cube state.
fn scramble() -> impl Strategy<Value = Vec<Rotation>> {
    prop::collection::vec(rotation(), 0..20)
}

/// Snapshots the full cube state, so it can be compared before/after a move sequence that should be a no-op.
fn state(c: &Cube) -> ([u8; 8], [u8; 8], [u8; 12], [u8; 12]) {
    (
        *c.corner_permutations(),
        *c.corner_orientations(),
        *c.edge_permutations(),
        *c.edge_orientations(),
    )
}

/// Applies the given moves to a fresh, solved cube.
fn apply(moves: &[Rotation]) -> Cube {
    let mut c = Cube::new();

    for m in moves {
        c.rotate(*m);
    }

    c
}

proptest! {
    /// Applying any move, then its inverse, should leave the cube exactly as it was - regardless of the state the
    /// cube started in.
    #[test]
    fn move_then_inverse_is_identity(scramble in scramble(), mv in rotation()) {
        let mut c = apply(&scramble);
        let before = state(&c);

        c.rotate(mv);
        c.rotate(mv.inverse());

        prop_assert_eq!(state(&c), before);
    }

    /// Applying any quarter turn four times should leave the cube exactly as it was.
    #[test]
    fn quarter_turn_four_times_is_identity(scramble in scramble(), mv in quarter_turn()) {
        let mut c = apply(&scramble);
        let before = state(&c);

        for _ in 0..4 {
            c.rotate(mv);
        }

        prop_assert_eq!(state(&c), before);
    }

    /// Applying any half turn twice should leave the cube exactly as it was.
    #[test]
    fn half_turn_twice_is_identity(scramble in scramble(), mv in half_turn()) {
        let mut c = apply(&scramble);
        let before = state(&c);

        c.rotate(mv);
        c.rotate(mv);

        prop_assert_eq!(state(&c), before);
    }

    /// Undoing any scramble, by applying the inverse of each move in reverse order, should always solve the cube.
    #[test]
    fn undoing_a_scramble_solves_the_cube(scramble in scramble()) {
        let mut c = apply(&scramble);

        for m in scramble.iter().rev() {
            c.rotate(m.inverse());
        }

        prop_assert!(c.solved());
    }
}
