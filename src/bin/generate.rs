use std::time::Instant;

use thwaite::cube::Cube;
use thwaite::solver::kociemba::{phase_one, phase_two};
use thwaite::solver::tables;
use thwaite::solver::thistlewaite::{group_one, group_three, group_two, group_zero};

fn main() -> std::io::Result<()> {
    println!("Generating Thistlewaite group_zero, this may take a while...");
    let start = Instant::now();
    let table = group_zero::Table::new();
    assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
    tables::write("src/solver/thistlewaite/group_zero/table.db", &table)?;
    println!("Generated Thistlewaite group_zero in {:?}", start.elapsed());

    println!("Generating Thistlewaite group_one, this may take a while...");
    let start = Instant::now();
    let table = group_one::Table::new();
    assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
    tables::write("src/solver/thistlewaite/group_one/table.db", &table)?;
    println!("Generated Thistlewaite group_one in {:?}", start.elapsed());

    // NOTE: no solved-depth assertion here - G2's table is seeded from 96 corner-permutation-orbit
    // representatives (see `group_two::initial`), not from the solved cube alone, so the solved cube's
    // own coordinate doesn't necessarily land on that coordinate's depth-0 anchor the way it does for
    // the other five (single-seed) tables.
    println!("Generating Thistlewaite group_two, this may take a while...");
    let start = Instant::now();
    let table = group_two::Table::new();
    tables::write("src/solver/thistlewaite/group_two/table.db", &table)?;
    println!("Generated Thistlewaite group_two in {:?}", start.elapsed());

    println!("Generating Thistlewaite group_three, this may take a while...");
    let start = Instant::now();
    let table = group_three::Table::new();
    assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
    tables::write("src/solver/thistlewaite/group_three/table.db", &table)?;
    println!("Generated Thistlewaite group_three in {:?}", start.elapsed());

    println!("Generating Kociemba phase_one, this may take a while...");
    let start = Instant::now();
    let table = phase_one::Table::new();
    assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
    tables::write("src/solver/kociemba/phase_one/table.db", &table)?;
    println!("Generated Kociemba phase_one in {:?}", start.elapsed());

    println!("Generating Kociemba phase_two, this may take a while...");
    let start = Instant::now();
    let table = phase_two::Table::new();
    assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
    tables::write("src/solver/kociemba/phase_two/table.db", &table)?;
    println!("Generated Kociemba phase_two in {:?}", start.elapsed());

    Ok(())
}
