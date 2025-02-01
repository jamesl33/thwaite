#![allow(dead_code)]

#[macro_use]
extern crate arrayref;

mod cube;
use cube::*;

mod solver;
use solver::*;

// fn main() -> std::io::Result<()> {
//     solver::tables::write("./src/solver/group_zero/table.db", &solver::group_zero::Table::new())?;
//     solver::tables::write("./src/solver/group_one/table.db", &solver::group_one::Table::new())?;
//     solver::tables::write("./src/solver/group_two/table.db", &solver::group_two::Table::new())?;
//     solver::tables::write("./src/solver/group_three/table.db", &solver::group_three::Table::new())?;
//
//     Ok(())
// }

fn main() {
    let mut c: Cube = Cube::new();

    let moves: Vec<Rotation> = rand::random_iter::<Rotation>().take(20).collect();

    println!("Scramble: {:?}", moves);

    for i in 0..moves.len() {
        c.rotate(moves[i])
    }

    let mut s: Solver = Solver::new(c);

    let solution = s.solve().unwrap();

    for i in 0..solution.len() {
        c.rotate(solution[i]);
    }

    assert!(c.solved());

    println!("Solution: {:?}", solution);
}
