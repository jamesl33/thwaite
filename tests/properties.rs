//! Property tests asserting that solving a scrambled cube, by any solver, always returns it to the solved state.

use proptest::prelude::*;
use thwaite::cube::{Cube, Rotation};
use thwaite::solver::kociemba::KociembaSolver;
use thwaite::solver::ThistlewaiteSolver;

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

/// A strategy producing a sequence of rotations, used to reach an arbitrary cube state.
fn scramble() -> impl Strategy<Value = Vec<Rotation>> {
    prop::collection::vec(rotation(), 0..20)
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
    // Solving is orders of magnitude slower than the pure cube-algebra checks in thwaite-core, so cap the number
    // of cases to keep the suite fast.
    #![proptest_config(ProptestConfig::with_cases(32))]

    /// Solving any scramble with the Kociemba solver, then applying the returned solution, should solve the cube.
    #[test]
    fn kociemba_solves_any_scramble(scramble in scramble()) {
        let mut c = apply(&scramble);

        let solution = KociembaSolver::new(c).solve().expect("scrambled cube should be solvable");

        for m in &solution {
            c.rotate(*m);
        }

        prop_assert!(c.solved());
    }

    /// Solving any scramble with the Thistlewaite solver, then applying the returned solution, should solve the
    /// cube.
    #[test]
    fn thistlewaite_solves_any_scramble(scramble in scramble()) {
        let mut c = apply(&scramble);

        let solution = ThistlewaiteSolver::new(c).solve().expect("scrambled cube should be solvable");

        for m in &solution {
            c.rotate(*m);
        }

        prop_assert!(c.solved());
    }
}
