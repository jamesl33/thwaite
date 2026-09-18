use std::sync::LazyLock;

use thwaite_core::cube;
use thwaite_core::cube::Rotation;
use thwaite_core::solver::group::Group;
use thwaite_core::solver::search::idas;
use thwaite_core::solver::tables;
use thwaite_core::solver::thistlewaite::{group_one, group_three, group_two, group_zero};

/// The pre-compute pattern database for traversing to G1.
///
/// Generated at build time; see `tables::read`.
static G0: LazyLock<group_zero::Table> =
    LazyLock::new(|| tables::read(include_bytes!(concat!(env!("OUT_DIR"), "/thistlewaite/group_zero/table.db"))));

/// The pre-compute pattern database for traversing to G2.
///
/// Generated at build time; see `tables::read`.
static G1: LazyLock<group_one::Table> =
    LazyLock::new(|| tables::read(include_bytes!(concat!(env!("OUT_DIR"), "/thistlewaite/group_one/table.db"))));

/// The pre-compute pattern database for traversing to G3.
///
/// Generated at build time; see `tables::read`.
static G2: LazyLock<group_two::Table> =
    LazyLock::new(|| tables::read(include_bytes!(concat!(env!("OUT_DIR"), "/thistlewaite/group_two/table.db"))));

/// The pre-compute pattern database for traversing to G4.
///
/// Generated at build time; see `tables::read`.
static G3: LazyLock<group_three::Table> =
    LazyLock::new(|| tables::read(include_bytes!(concat!(env!("OUT_DIR"), "/thistlewaite/group_three/table.db"))));

/// Exposes an API to solve the Rubik's Cube using the Thistlewaite-45 method.
#[derive(Debug)]
pub struct ThistlewaiteSolver {
    /// The cube being solved.
    cube: cube::Cube,
}

impl ThistlewaiteSolver {
    /// Returns a new solver, which will solve the given cube.
    pub fn new(cube: cube::Cube) -> ThistlewaiteSolver {
        ThistlewaiteSolver { cube }
    }

    /// Returns a solution for the target cube, if one can be found.
    pub fn solve(&mut self) -> Option<Vec<cube::Rotation>> {
        // Already solved, no moves required
        if self.cube.solved() {
            return Some(vec![]);
        }

        // Calculate the rotations to move to G1
        let zero = idas(self.cube, Group::Zero.moves(), &|cube| G0.depth(cube))?;

        // Apply those moves
        self.apply(&zero);

        // Calculate the rotations to move to G1
        let one = idas(self.cube, Group::One.moves(), &|cube| G1.depth(cube))?;

        // Apply the moves
        self.apply(&one);

        // Calculate the rotations to move to G2
        let two = idas(self.cube, Group::Two.moves(), &|cube| G2.depth(cube))?;

        // Apply the moves
        self.apply(&two);

        // Calculate the rotations to move to G3
        let three = idas(self.cube, Group::Three.moves(), &|cube| G3.depth(cube))?;

        // Apply the moves
        //
        // NOTE: This is not strictly required, however, allows us to leave the cube in the solved state.
        self.apply(&three);

        Some([zero, one, two, three].concat())
    }

    /// Applies the given moves to the cube.
    fn apply(&mut self, moves: &[Rotation]) {
        for i in 0..moves.len() {
            self.cube.rotate(moves[i]);
        }
    }
}
