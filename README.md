# thwaite

`thwaite` is a Rubik's Cube solver, written in Rust named after a combination of **Thwaite**, beautiful a village in the Yorkshire Dales, on The Pennine Way and **Morwen Thistlethwaite** (the creator of the implemented solution methodology).

# Usage

In its current state, when build and run, `thwaite` will generate a scrambled Rubik's Cube, the immediately solve it; there's no method to input a pre-scrambled cube.

```
$ cargo run --release
Scramble: [BP, RP, RP, R2, D, U2, B, U, F2, F, B, U2, D, D, RP, F2, D, D, D, FP]
Solution: [U, F2, R, B2, U, FP, L, U2, F, L, B2, U2, B2, L, F, L, F2, LP, F2, R2, F2, L, F2, L, U2, L2, U2, L2, U2, B2, R2, U2, L2, U2, L2, F2, L2]
```

# Performance

`thwaite` is built to be performant:

- Pre-computed lookup tables (embedded, snappy compressed data)
- Pre-computed factorials/combinations
- Depth first search (DFS) rather than breadth first search (BFS)

I've not run into many cube states which take longer than $250ms$ to solve; I've not generated the deepest possible tables though, so that may be a low-hanging fruit improvement.

```
$ hyperfine --warmup 5 --runs 250 ./target/release/thwaite
Benchmark 1: ./target/release/thwaite
  Time (mean ± σ):      70.3 ms ±  33.9 ms    [User: 60.1 ms, System: 9.1 ms]
  Range (min … max):    37.8 ms … 222.0 ms    250 runs
```


# Cube Representation

Rubik's Cubes can be represented in many different ways; to name a few:

- **Facelet:** A 1x54 array, manipulated manually.
- **Coordinates:** $x$, $y$, $z$ coordinates that are manipulated using rotation matrices.

Each has is merits/use-case; this solver represents a Cube using the format described in [this](http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf) paper.

## Permutations

The permutation of corner pieces can be represented as a 2x8 array, where the first row represents the corners 1-8; the second holding the actual positions of those pieces.

| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |

With that in-mind, the first array may be omitted as it's static (implied) in future references.

| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| --- | --- | --- | --- | --- | --- | --- | --- |

For example, the $U$ permutation, may be represented using the following array.

| 4   | 1   | 2   | 7   | 3   | 5   | 6   | 0   |
| --- | --- | --- | --- | --- | --- | --- | --- |

The same theory applies to edges, except a 2x12 array is used instead.

## Orientations

The orientation for corners can be represented as a 2x8 array, where the first row represents the corners 1-8; the second holding their orientations (0-3).

| 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |

- **0:** Correctly Oriented
- **1:** Twisted Clockwise
- **2:** Twisted Counter-Clockwise

As with permutations, the first array may be omitted; the first is implied.

| 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   |
| --- | --- | --- | --- | --- | --- | --- | --- |

For example, the $U$ permutation, may be represented using the following array.

| -1  | 0   | 0   | -1  | 1   | 0   | 0   | 1   |
| --- | --- | --- | --- | --- | --- | --- | --- |

The orientation of each piece then becomes $(cur[i] * rot[i]) \mod n$ where $n$ is the number of orientations; two for edges, three for corners.

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

The generation for the pattern databases uses a limited depth first search (DFS) where for each group, a search is started, using the groups valid moves from the solved cube; the depth is then recorded in the lookup table.

### Indexing

By far the most complex (intricate) part of the solver, is the indexing of cube-state into the pattern databases; in most cases, the permutations (or orientations) of a subset of the cube pieces are turned into indices where a "depth" is stored.

The complexity from indexing cube state can be somewhat side-stepped, by storing the entire cube state (e.g. as a string) with a depth, however, this will result in tables in the realms of tens of megabytes.

# References

With this implementation, I'm simply standing on the shoulders of giants; it would not have been possible without a huge number of resources.

- http://joren.ralphdesign.nl/projects/rubiks_cube/cube.pdf
- https://cube.stanford.edu/class/files/rokicki_cubecomp.pdf
- https://en.wikipedia.org/wiki/Iterative_deepening_A*
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
- [ ] Document in-detail, pruning table numbers, creation and indexing strategies
