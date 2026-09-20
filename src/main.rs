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

    /// Target cube state; when given, the printed solution takes `cube` to this state instead of to solved.
    #[arg(long)]
    target: Option<String>,

    /// Which solving algorithm to use.
    #[arg(long, value_enum, default_value = "thistlewaite")]
    algorithm: Solver,
}

/// Returns the cube to solve, which will be provided by the user or randomly scrambled. Prints the scramble
/// sequence when one is generated.
fn cube(state: Option<String>) -> Cube {
    match state {
        Some(state) => Cube::from(state.as_str()),
        None => {
            let (cube, moves) = Cube::scrambled(&mut rand::rng());
            println!("Scramble: {:?}", moves);
            cube
        }
    }
}

/// Solves the given cube, returning the moves required to reach the solved state.
fn solve(c: Cube, algorithm: &Solver) -> Vec<Rotation> {
    match algorithm {
        Solver::Thistlewaite => ThistlewaiteSolver::new(c).solve(),
        Solver::Kociemba => KociembaSolver::new(c).solve(),
    }
    .expect("cube is not solvable; check the provided cube string")
}

/// Reverses a solution, so applying the result undoes the original moves in the opposite order.
fn reverse(solution: &[Rotation]) -> Vec<Rotation> {
    solution.iter().rev().map(Rotation::inverse).collect()
}

/// Returns the moves required to solve `start`.
fn solve_to_solved(start: Cube, algorithm: &Solver) -> Vec<Rotation> {
    let solution = solve(start, algorithm);

    let mut c = start;

    for m in &solution {
        c.rotate(*m);
    }

    assert!(c.solved());

    solution
}

/// Returns the shortest moves that take `start` to `target`.
fn solve_to_target(start: Cube, target: Cube, algorithm: &Solver) -> Vec<Rotation> {
    let to_solved = solve(start, algorithm);

    // Solving the target and reversing that solution gives the moves from solved to the target.
    let to_target = reverse(&solve(target, algorithm));

    // Apply both move sets to a fresh, solved cube: the result is the state whose shortest solution
    // is exactly the shortest path from `start` to `target`. Reversing that solution gives the
    // shortest `start` -> `target` path, rather than the (longer) concatenation of the two solves.
    let mut delta = Cube::new();

    for m in to_solved.iter().chain(to_target.iter()) {
        delta.rotate(*m);
    }

    let solution = reverse(&solve(delta, algorithm));

    let mut c = start;

    for m in &solution {
        c.rotate(*m);
    }

    assert_eq!(c.corner_permutations(), target.corner_permutations());
    assert_eq!(c.corner_orientations(), target.corner_orientations());
    assert_eq!(c.edge_permutations(), target.edge_permutations());
    assert_eq!(c.edge_orientations(), target.edge_orientations());

    solution
}

fn main() {
    let args = Args::parse();

    let start = cube(args.cube);

    let solution = match args.target {
        Some(target) => solve_to_target(start, Cube::from(target.as_str()), &args.algorithm),
        None => solve_to_solved(start, &args.algorithm),
    };

    println!("Solution: {:?}", solution);
}
