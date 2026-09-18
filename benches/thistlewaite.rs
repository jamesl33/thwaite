use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::rngs::StdRng;
use rand::SeedableRng;

use thwaite::cube::Cube;
use thwaite::solver::ThistlewaiteSolver;

fn solve(c: &mut Criterion) {
    // Fixed seed: same scramble every run, so results are comparable across benchmarks.
    let seeded = Cube::scrambled(&mut StdRng::seed_from_u64(0));

    c.bench_function("thistlewaite solve (seeded)", |b| {
        b.iter_batched(
            || seeded,
            |cube| ThistlewaiteSolver::new(black_box(cube)).solve(),
            BatchSize::SmallInput,
        )
    });

    // Fresh scramble per sample, so results reflect variance in solve time across cube states.
    c.bench_function("thistlewaite solve (random)", |b| {
        b.iter_batched(
            || Cube::scrambled(&mut rand::rng()),
            |cube| ThistlewaiteSolver::new(black_box(cube)).solve(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, solve);
criterion_main!(benches);
