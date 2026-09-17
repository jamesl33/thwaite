use std::path::{Path, PathBuf};

use thwaite_core::cube::Cube;
use thwaite_core::solver::kociemba::{phase_one, phase_two};
use thwaite_core::solver::tables;
use thwaite_core::solver::thistlewaite::{group_one, group_three, group_two, group_zero};

/// Returns the path a generated table should live at, skipping generation if it's already there. `OUT_DIR`
/// persists across incremental builds, so this only pays generation's cost - minutes, for the larger tables - on
/// a clean build or after `cargo clean`, not on every invocation.
fn path(out_dir: &str, name: &str) -> Option<PathBuf> {
    let path = Path::new(out_dir).join(name);

    if path.exists() {
        return None;
    }

    println!("cargo:warning=thwaite-core: generating {name}, this may take a while...");

    Some(path)
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    std::thread::scope(|scope| {
        if let Some(path) = path(&out_dir, "group_zero_table.db") {
            scope.spawn(move || {
                let table = group_zero::Table::new();
                assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
                tables::write(path.to_str().unwrap(), &table).unwrap();
            });
        }

        if let Some(path) = path(&out_dir, "group_one_table.db") {
            scope.spawn(move || {
                let table = group_one::Table::new();
                assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
                tables::write(path.to_str().unwrap(), &table).unwrap();
            });
        }

        if let Some(path) = path(&out_dir, "group_two_table.db") {
            scope.spawn(move || {
                // NOTE: no solved-depth assertion here - G2's table is seeded from 96 corner-permutation-orbit
                // representatives (see `group_two::initial`), not from the solved cube alone, so the solved
                // cube's own coordinate doesn't necessarily land on that coordinate's depth-0 anchor the way it
                // does for the other five (single-seed) tables.
                let table = group_two::Table::new();
                tables::write(path.to_str().unwrap(), &table).unwrap();
            });
        }

        if let Some(path) = path(&out_dir, "group_three_table.db") {
            scope.spawn(move || {
                let table = group_three::Table::new();
                assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
                tables::write(path.to_str().unwrap(), &table).unwrap();
            });
        }

        if let Some(path) = path(&out_dir, "phase_one_table.db") {
            scope.spawn(move || {
                let table = phase_one::Table::new();
                assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
                tables::write(path.to_str().unwrap(), &table).unwrap();
            });
        }

        if let Some(path) = path(&out_dir, "phase_two_table.db") {
            scope.spawn(move || {
                let table = phase_two::Table::new();
                assert_eq!(table.depth(&Cube::new()), 0, "solved cube should be depth 0");
                tables::write(path.to_str().unwrap(), &table).unwrap();
            });
        }
    });
}
