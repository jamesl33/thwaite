use clap::{Parser, ValueEnum};

use thwaite::cube::*;
use thwaite::solver::*;

/// Which solving algorithm to use.
#[derive(Clone, ValueEnum)]
enum Solver {
    Thistlewaite,
    Kociemba,
}

/// A Rubik's Cube solver.
#[derive(Parser)]
struct Args {
    /// Cube state to solve; a random scramble is generated if omitted.
    cube: Option<String>,

    /// Which solving algorithm to use.
    #[arg(long, value_enum, default_value = "thistlewaite")]
    algorithm: Solver,
}

/// Returns a scrambled cube.
fn scramble() -> Cube {
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

    c
}

/// Returns the cube to solve, which will be provided by the user or randomly scrambled.
fn cube(state: Option<String>) -> Cube {
    match state {
        Some(state) => Cube::from(state.as_str()),
        None => scramble(),
    }
}

fn main() {
    let args = Args::parse();

    let mut c = cube(args.cube);

    let solution = match args.algorithm {
        Solver::Thistlewaite => ThistlewaiteSolver::new(c).solve(),
        Solver::Kociemba => KociembaSolver::new(c).solve(),
    }
    .expect("cube is not solvable; check the provided cube string");

    for i in 0..solution.len() {
        c.rotate(solution[i]);
    }

    assert!(c.solved());

    println!("Solution: {:?}", solution);
}
