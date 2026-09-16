use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use thwaite::cube::{Cube, Rotation};
use thwaite::solver::kociemba::KociembaSolver;

/// Returns a scrambled cube, using the given RNG.
fn scrambled<R: Rng>(rng: &mut R) -> Cube {
    let mut c = Cube::new();

    let mut moves = 0;
    while moves < 20 {
        let mv: Rotation = rng.random();

        if c.redundant(&mv) {
            continue;
        }

        c.rotate(mv);
        moves += 1;
    }

    c
}

fn solve(c: &mut Criterion) {
    // Fixed seed: same scramble every run, so results are comparable across benchmarks.
    let seeded = scrambled(&mut StdRng::seed_from_u64(0));

    c.bench_function("kociemba solve (seeded)", |b| {
        b.iter_batched(
            || seeded,
            |cube| KociembaSolver::new(black_box(cube)).solve(),
            BatchSize::SmallInput,
        )
    });

    // Fresh scramble per sample, so results reflect variance in solve time across cube states.
    //
    // NOTE: this MVP's phase two heuristic can be far too optimistic for some cube states (see
    // `KociembaSolver`'s docs), so this benchmark's variance is expected to be much wider than
    // Thistlewaite's.
    c.bench_function("kociemba solve (random)", |b| {
        b.iter_batched(
            || scrambled(&mut rand::rng()),
            |cube| KociembaSolver::new(black_box(cube)).solve(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, solve);
criterion_main!(benches);
