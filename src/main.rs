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
    let mut scramble: Vec<Rotation> = Vec::with_capacity(20);

    loop {
        if scramble.len() >= 20 {
            break;
        }

        let mv: Rotation = rand::random();

        if c.redundant(&mv) {
            continue;
        }

        c.rotate(mv);

        scramble.push(mv);
    }

    println!("Scramble: {:?}", scramble);

    let mut s: Solver = Solver::new(c);

    let solution = s.solve().unwrap();

    for i in 0..solution.len() {
        c.rotate(solution[i]);
    }

    assert!(c.solved());

    println!("Solution: {:?}", solution);
}
