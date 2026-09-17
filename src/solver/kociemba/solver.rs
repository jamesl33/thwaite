use std::sync::LazyLock;

use crate::cube;
use crate::cube::Rotation;
use crate::solver;
use crate::solver::kociemba::phase::{PHASE_ONE_VALID_MOVES, PHASE_TWO_VALID_MOVES};
use crate::solver::kociemba::{phase_one, phase_two};
use crate::solver::search::idas;

/// The pre-computed pattern database for traversing to phase one's target group (see `phase::PHASE_TWO_VALID_MOVES`
/// for the group's generators, and why they differ from the textbook `<U, D, L2, R2, F2, B2>`).
///
/// Lazily decoded once per process, rather than once per `solve()` call.
static P1: LazyLock<phase_one::Table> =
    LazyLock::new(|| solver::tables::read(include_bytes!("./phase_one/table.db")));

/// The pre-computed pattern database for solving the cube, once already in phase one's target group.
///
/// Lazily decoded once per process, rather than once per `solve()` call.
static P2: LazyLock<phase_two::Table> =
    LazyLock::new(|| solver::tables::read(include_bytes!("./phase_two/table.db")));

/// Exposes an API to solve the Rubik's Cube using Kociemba's two-phase method.
///
/// https://kociemba.org/cube.htm
///
/// NOTE: Phase one returns the first IDA* solution found (not one of several candidates chosen to make phase two
/// easy), so phase two's IDA* search time varies with the scramble - empirically, across 200,000 random 20-move
/// scrambles, mean ~2.6ms and worst case ~630ms; no adversarial worst case has been established, since
/// multi-candidate phase one search isn't implemented. Phase two's heuristic is the max of three tables: two
/// independently generated coordinate tables (corner-permutation and edge-permutation, each paired with the
/// LR-slice edge permutation - only weakly correlated with each other) plus a genuinely joint
/// corner/edge-permutation coordinate, made tractable to store by reducing it with `cube::SYMMETRIES` (see
/// `phase_two::table::CORNER_SYM`). Production two-phase implementations also retry phase one with multiple
/// candidates to make phase two easier still; that isn't implemented here.
#[derive(Debug)]
pub struct KociembaSolver {
    /// The cube being solved.
    cube: cube::Cube,
}

impl KociembaSolver {
    /// Returns a new solver, which will solve the given cube.
    pub fn new(cube: cube::Cube) -> KociembaSolver {
        KociembaSolver { cube }
    }

    /// Returns a solution for the target cube, if one can be found.
    pub fn solve(&mut self) -> Option<Vec<cube::Rotation>> {
        // Already solved, no moves required
        if self.cube.solved() {
            return Some(vec![]);
        }

        // Calculate the rotations to reach phase one's target group
        let one = idas(self.cube, &PHASE_ONE_VALID_MOVES, &|cube| P1.depth(cube))?;

        // Apply those moves
        self.apply(&one);

        // Calculate the rotations to solve the cube, using only phase one's target group's moves
        let two = idas(self.cube, &PHASE_TWO_VALID_MOVES, &|cube| P2.depth(cube))?;

        // Apply the moves
        self.apply(&two);

        Some([one, two].concat())
    }

    /// Applies the given moves to the cube.
    fn apply(&mut self, moves: &[Rotation]) {
        for i in 0..moves.len() {
            self.cube.rotate(moves[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::Cube;

    #[test]
    fn solves_an_already_solved_cube() {
        let mut solver = KociembaSolver::new(Cube::new());

        assert_eq!(solver.solve(), Some(vec![]));
    }

    #[test]
    fn solves_a_scrambled_cube() {
        let mut cube = Cube::new();

        let scramble = [
            Rotation::R,
            Rotation::U,
            Rotation::F2,
            Rotation::L,
            Rotation::D2,
            Rotation::B,
            Rotation::RP,
            Rotation::U2,
            Rotation::F,
            Rotation::LP,
            Rotation::D,
            Rotation::B2,
            Rotation::R2,
            Rotation::UP,
            Rotation::F2,
            Rotation::L2,
            Rotation::D2,
            Rotation::B,
        ];

        for mv in scramble {
            cube.rotate(mv);
        }

        let mut solver = KociembaSolver::new(cube);
        let solution = solver.solve().expect("cube should be solvable");

        for mv in &solution {
            cube.rotate(*mv);
        }

        assert!(cube.solved());
    }
}
