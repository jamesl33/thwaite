# thwaite

`thwaite` is a Rubik's Cube solver, written in Rust named after a combination of **Thwaite**, a beautiful village in the Yorkshire Dales on The Pennine Way and **Morwen Thistlethwaite** (the creator of the implemented algorithm).

# Usage

In its current state, when built and run, `thwaite` supports two use-cases:

1. Generating a random cube, then solving it
2. Solving a provided cube

```
$ cargo run --release
Scramble: [BP, RP, RP, R2, D, U2, B, U, F2, F, B, U2, D, D, RP, F2, D, D, D, FP]
Solution: [U, F2, R, B2, U, FP, L, U2, F, L, B2, U2, B2, L, F, L, F2, LP, F2, R2, F2, L, F2, L, U2, L2, U2, L2, U2, B2, R2, U2, L2, U2, L2, F2, L2]
```

```
$ cargo run --release 'OOWYYBBWOBYGRROGGRROWGBBOYWGROYWRGRYYGWBOWYOYRGBBGWBWR'
Solution: [BP, R2, U, L2, FP, U2, RP, D2, B, R2, F, L, F, LP, F2, L, B2, R, F2, R, F2, L, F2, D2, R2, B2, L2, U2, F2, L2, U2, F2, L2, U2]
```

# Performance

`thwaite` is built to be performant:

- Pre-computed lookup tables (embedded, snappy compressed data; see [Generation](#generation) for how they're built)
- Pre-computed factorials/combinations
- Solves via iterative deepening A\* (IDA\*) - a depth-first, heuristic-guided search, appropriate here since it's goal-directed rather than exhaustive (see [IDA\*](#ida-iterative-deepening-a))

I've not run into many cube states which take longer than $250ms$ to solve; I've not generated the deepest possible tables though, so that may be a low-hanging fruit improvement.

The `cargo bench` suite ([`benches/thistlewaite.rs`](./benches/thistlewaite.rs)) measures `ThistlewaiteSolver::solve` in isolation (table load included, process start-up excluded); once against a fixed (seeded) scramble for run-to-run comparability, and once against a fresh scramble per sample to capture variance across cube states:

```
$ cargo bench
thistlewaite solve (seeded)
                        time:   [72.418 ms 73.032 ms 73.672 ms]
thistlewaite solve (random)
                        time:   [57.824 ms 60.561 ms 63.549 ms]
```


# Cube Representation

Rubik's Cubes can be represented in many different ways; to name a few:

- **Facelet:** A 1x54 array, manipulated manually.
- **Coordinates:** $x$, $y$, $z$ coordinates that are manipulated using rotation matrices.

Each has is merits/use-case; this solver represents a Cube using the format described in [this](http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf) ([archived](https://web.archive.org/web/20240512092325/http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf)) paper.

## Permutations

The permutation of corner pieces can be represented as a 2x8 array, where the first row represents the corners 1-8; the second holding the actual positions of those pieces.

<div align="center">

| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |

</div>

With that in-mind, the first array may be omitted as it's static (implied) in future references.

<div align="center">

| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| --- | --- | --- | --- | --- | --- | --- | --- |

</div>

For example, the $U$ permutation, may be represented using the following array.

<div align="center">

| 4   | 1   | 2   | 7   | 3   | 5   | 6   | 0   |
| --- | --- | --- | --- | --- | --- | --- | --- |

</div>

The same theory applies to edges, except a 2x12 array is used instead.

## Orientations

The orientation for corners can be represented as a 2x8 array, where the first row represents the corners 1-8; the second holding their orientations (0-3).

<div align="center">

| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |

</div>

- **0:** Correctly Oriented
- **1:** Twisted Clockwise
- **2:** Twisted Counter-Clockwise

As with permutations, the first array may be omitted; the first is implied.

<div align="center">

| 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
| --- | --- | --- | --- | --- | --- | --- | --- |

</div>

For example, the $U$ permutation, may be represented using the following array.

<div align="center">

| -1  | 0   | 0   | -1  | 1   | 0   | 0   | 1   |
| --- | --- | --- | --- | --- | --- | --- | --- |

</div>

The orientation of each piece then becomes $((cur[i] * rot[i]) \mod n)$ where $n$ is the number of orientations; two for edges, three for corners.

## Cube

The Cube is made up of four arrays:

- **Corner Permutations:** The positions of the corner pieces.
- **Edge Permutations:** The positions of the edge pieces.
- **Corner Orientations:** The orientation of the corner pieces.
- **Edge Orientations:** The orientations of the edge pieces.

With each using the representations depicted above.

# Solver

The solving algorithm implemented in `thwaite` is [Thistlewaite 45](https://www.jaapsch.net/puzzles/thistle.htm), an algorithm which uses group theory to limit the search space for a solution by progressively restricting the available moves (after reaching certain states).

## Group 0 (G0)

<p align="center">
$G0 = \{L,R,F,B,U,D\}$
</p>

The furthest group from a solution, where all $18$ moves are available[^1][^2] which has $4.33\times10^{19}$ valid states, and a pruning table with $2,048$ entries.

[^1]: Slices and cube rotations are excluded.
[^2]: Prime (reverse) rotations, and 180 degree rotations are valid.

## Group 1 (G1)

<p align="center">
$G1 = \{L,R,F,B,U2,D2\}$
</p>

The first group with move limitations, $14$ moves are not available; there are now $2.11\times10^{16}$ valid states, and a pruning table with $1,082,565$ entries.

## Group 2 (G2)

<p align="center">
$G2 = \{L,R,F2,B2,U2,D2\}$
</p>

The third group, now contains $10$ valid moves which has $1.95\times10^{10}$ valid states, and a pruning table with $2,822,400$ entries.

## Group 3 (G3)

<p align="center">
$G3 = \{L2,R2,F2,B2,U2,D2\}$
</p>

The final (non-solved) group, which restricts valid moves to only $180\degree$ moves which has $6.63\times1^{05}$ valid states, and a pruning table with $663,552$ entries.

## Group 4 (G4)

<p align="center">
$G4 = \{I\}$
</p>

The solved (identity) state, with only one valid state.

## IDA\* (Iterative Deepening A\*)

The solver utilises iterative deepening A\* (IDA\*) as described on [Wikipedia](https://en.wikipedia.org/wiki/Iterative_deepening_A*) which uses a heuristic (the "depth" from the solved state) to traverse the search space extremely quickly.

## Pattern Databases

The "depth" from the solved state, is pre-computed for each group and stored in a lookup table where the index represents a cube state[^3], and the value at the index, the depth.

[^3]: More accurately, a subset of the cube-state; only that which we care about to reach the goal state for the group.

### Generation

G2 and G3's tables are generated with a breadth first search (BFS): a search is started using the group's valid moves from the solved cube, visiting states in non-decreasing depth order, so the first time a state is reached is guaranteed to be its shortest depth[^4]; deduplicating on `(coordinate, last move face)` - rather than just the coordinate, since `redundant()`'s next-move eligibility depends on the last move's face - keeps the search exhaustive while still visiting each pair at most once. This is only safe when the group's coordinate is "closed" under the move action: two states sharing a coordinate must always transition to the same next coordinate for a given move, otherwise this dedup silently under-fills the table. G2 and G3's coordinates (which both fold in permutation/combination information, not just orientation) satisfy this; both Kociemba phases' coordinates do too.

G0 and G1 don't: their coordinates are orientation-only (G0) or orientation combined with only a partial permutation combination (G1), which isn't enough to determine a state's next coordinate uniquely. They're generated with a depth-limited depth first search instead[^5], recording the shallower of any two depths found for the same coordinate - correct regardless of how much information the coordinate captures, just more expensive; a real difference in practice, since promoting G0/G1's search to G2/G3's BFS was tried and confirmed (by comparing against this older search's output) to leave most of G0's table at its sentinel depth, never visited.

G2's table has a further exception: corner permutation parity isn't fully fixed by G0/G1, so a single BFS from the solved cube can't reach every reachable corner-permutation orbit within a sane depth limit. Instead, a shallow (depth $4$) depth first search first collects $96$ distinct initial cube states (one per valid corner-permutation orbit, keyed by [`ptoidx`](#indexing)), then the BFS is seeded from all $96$ of those states at once, rather than just the solved cube.

[^4]: A plain depth first search re-explores the same state through every move sequence that reaches it, which is exponential in the search depth; BFS turns this into roughly `states * branching factor`. The group's known max depth (sourced from the same paper as the group descriptions above) is still used as the search's sentinel/cutoff value.

[^5]: Depth-limited to the group's known max depth, same source as above.

The checked-in `table.db` files (one per group, plus Kociemba's two phases) are pre-generated; regenerate them, if the generation algorithm or indexing changes, with:

```
$ cargo run --release --bin generate
```

This is slow (large tables can take a while to build) and only needs to be run when generation itself changes, not as part of a normal build.

### Indexing

By far the most complex (intricate) part of the solver, is the indexing of cube-state into the pattern databases; in most cases, the permutations (or orientations) of a subset of the cube pieces are turned into indices where a "depth" is stored.

The complexity from indexing cube state can be somewhat side-stepped, by storing the entire cube state (e.g. as a string) with a depth, however, this will result in tables in the realms of tens of megabytes.

Two indexing primitives are reused across groups:

- **Orientation index:** treats the orientations of $n$ of a piece type's $n+1$ pieces (the last is implied by the others) as digits of a base-$k$ number, where $k$ is $2$ for edges, $3$ for corners.
- **Permutation index:** ranks a permutation of $n$ pieces as a Lehmer code, i.e. its position (0-indexed) amongst all $n!$ orderings; used where a piece subset's exact arrangement matters (e.g. G2/G3 corner permutations).
- **Combination index:** ranks which $k$ of $n$ positions a subset of pieces occupies (ignoring their order), amongst all $\binom{n}{k}$ selections; used where only which pieces have left/entered a set of positions matters (e.g. G1's LR-slice edges, G2's non-E-slice edge distribution).

Each group then combines the indices of the piece-state it fixes into a single flat table offset (`major * minor_size + minor`, extended to as many dimensions as needed):

- **G0:** Only edge orientations matter, encoded directly as an $11$-bit orientation index ($2^{11} = 2{,}048$ entries).
- **G1:** A combination index over the LR-slice edge permutations ($\binom{12}{4} = 495$, via a precomputed $0..2048 \to 0..495$ lookup table) combined with a corner orientation index ($3^7 = 2{,}187$), for $495 \times 2{,}187 = 1{,}082{,}565$ entries.
- **G2:** A permutation index over all corner permutations ($8! = 40{,}320$) combined with a combination index over the edge distribution across the $8$ non-E-slice positions ($\binom{8}{4} = 70$), for $40{,}320 \times 70 = 2{,}822{,}400$ entries.
- **G3:** Permutation/combination indices over the M-slice, S-slice and E-slice edges and the corner tetrad are ranked separately (via Lehmer codes) then folded together into a single fixed-size ($663{,}552$ entry) index; see [`group_three/table.rs`](src/solver/thistlewaite/group_three/table.rs) for the exact mixed-radix layout, which is adapted from [`itaysadeh/rubiks-cube-solver`](https://github.com/itaysadeh/rubiks-cube-solver).

# Kociemba (Experimental)

`thwaite` also includes an MVP [`KociembaSolver`](src/solver/kociemba/solver.rs) implementing [Kociemba's two-phase algorithm](https://kociemba.org/cube.htm). It is **not** wired into the CLI or used by default - it exists as a separate code path, exercised only by its own tests, alongside the default `ThistlewaiteSolver`.

Kociemba's algorithm reaches a solution in two phases rather than Thistlewaite's four groups:

- **Phase one** searches with all $18$ moves until edge orientation, corner orientation and LR-slice edge membership are simultaneously satisfied (analogous to Thistlewaite's G0+G1 combined into a single goal).
- **Phase two** then searches using only the moves that preserve that state, until the cube is solved.

Note that this codebase defines piece orientation relative to the L/R axis (`Cube::rotate_left`/`rotate_right` never touch orientation; `rotate_up`/`rotate_down` disturb both corner and edge orientation), the opposite of the textbook Kociemba convention (which assumes U/D quarter turns preserve orientation). Phase two's move set here is therefore `<L, R, F2, B2, U2, D2>`, not the textbook `<U, D, L2, R2, F2, B2>` - and the tracked "slice" is the same LR-slice edges (piece ids 8-11) Thistlewaite's own G1 already tracks, not a U/D-relative E-slice.

Both phases reuse the same [indexing primitives](#indexing) as the Thistlewaite groups (orientation index, Lehmer permutation index, combination index), duplicated locally per module rather than shared, matching this repo's existing per-group convention. Each phase's pruning table is too large to store as a single dense array (phase one's three coordinates alone would need ~2.2 billion entries), so each phase instead uses two independently-generated coordinate tables with the IDA* heuristic taken as their max - the same technique real two-phase implementations use for phase one.

**Known limitation:** this MVP does not implement symmetry reduction or multi-candidate phase-one search (both used by production two-phase implementations to keep phase two fast). Because phase two's two coordinates (corner permutation and edge permutation) are only weakly correlated, the max-of-two heuristic can be a poor lower bound for some cube states, so phase two's search time varies widely - anywhere from milliseconds to tens of seconds - depending on the scramble and which phase-one solution happened to be found first.

# References

With this implementation, I'm simply standing on the shoulders of giants; it would not have been possible without a huge number of resources.

- http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf (dead; archived: https://web.archive.org/web/20240512092325/http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf)
- https://cube.stanford.edu/class/files/rokicki_cubecomp.pdf
- https://en.wikipedia.org/wiki/Iterative_deepening_A%2A
- https://en.wikipedia.org/wiki/Optimal_solutions_for_the_Rubik%27s_Cube
- https://explainextended.com/2022/12/31/happy-new-year-14
- https://github.com/benbotto/rubiks-cube-cracker
- https://github.com/dfinnis/Rubik
- https://github.com/itaysadeh/rubiks-cube-solver
- https://iopscience.iop.org/article/10.1088/1742-6596/2386/1/012018/pdf
- https://kociemba.org/cube.htm
- https://www.cs.princeton.edu/courses/archive/fall06/cos402/papers/korfrubik.pdf
- https://www.jaapsch.net/puzzles/compindx.htm
- https://www.reddit.com/r/algorithms/comments/11lgnwi/how_would_you_optimally_store_large_pattern

A special mention to Joren Heit's paper "Building and Solving Rubik’s Cube in Mathworks R© Matlab R©." which is the basis for a huge amount of this work.

# Honourable Mentions

- https://github.com/jamesl33/pysolver

# TODO

- [ ] A CLI which allows inputting scrambled cubes
- [ ] Turn the crate in a library, rather than a binary
