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

    c.rotate(cube::Rotation::R);
    c.rotate(cube::Rotation::U);
    c.rotate(cube::Rotation::RP);
    c.rotate(cube::Rotation::UP);
    c.rotate(cube::Rotation::LP);
    c.rotate(cube::Rotation::UP);
    c.rotate(cube::Rotation::L);
    c.rotate(cube::Rotation::U);
    c.rotate(cube::Rotation::D2);
    c.rotate(cube::Rotation::B2);
    c.rotate(cube::Rotation::R2);
    c.rotate(cube::Rotation::F);
    c.rotate(cube::Rotation::L);
    c.rotate(cube::Rotation::B);

    let mut s: Solver = Solver::new(c);

    println!("{:?}", s.solve());
}
