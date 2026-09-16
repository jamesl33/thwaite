use crate::cube;
use crate::cube::Rotation;
use crate::solver;
use crate::solver::search::idas;

/// The pre-compute pattern database for traversing to G1.
static G0: &[u8] = include_bytes!("./group_zero/table.db");

/// The pre-compute pattern database for traversing to G2.
static G1: &[u8] = include_bytes!("./group_one/table.db");

/// The pre-compute pattern database for traversing to G3.
static G2: &[u8] = include_bytes!("./group_two/table.db");

/// The pre-compute pattern database for traversing to G4.
static G3: &[u8] = include_bytes!("./group_three/table.db");

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

        // Setup the G0 table
        let g0 = solver::tables::read::<super::group_zero::Table>(G0);

        // Calculate the rotations to move to G1
        let zero = idas(self.cube, solver::Group::Zero.moves(), &|cube| g0.depth(cube))?;

        // Apply those moves
        self.apply(&zero);

        // Setup the G1 table
        let g1 = solver::tables::read::<super::group_one::Table>(G1);

        // Calculate the rotations to move to G1
        let one = idas(self.cube, solver::Group::One.moves(), &|cube| g1.depth(cube))?;

        // Apply the moves
        self.apply(&one);

        // Setup the G2 table
        let g2 = solver::tables::read::<super::group_two::Table>(G2);

        // Calculate the rotations to move to G2
        let two = idas(self.cube, solver::Group::Two.moves(), &|cube| g2.depth(cube))?;

        // Apply the moves
        self.apply(&two);

        // Setup the G3 table
        let g3 = solver::tables::read::<super::group_three::Table>(G3);

        // Calculate the rotations to move to G3
        let three = idas(self.cube, solver::Group::Three.moves(), &|cube| g3.depth(cube))?;

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
