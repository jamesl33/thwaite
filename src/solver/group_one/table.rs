use std::collections::VecDeque;

use crate::cube::{Cube, NUM_EDGES};
use crate::solver::group::Group;

/// SIZE - TODO
const SIZE: usize = 2048;

/// Table - TODO
#[derive(Debug)]
pub struct Table {
    /// data - TODO
    data: [usize; SIZE],
}

impl Table {
    pub fn new() -> Table {
        g0()
    }
}

/// g0 - TODO
fn g0() -> Table {
    /// DEPTH - TODO
    const DEPTH: usize = 7;

    // We initialize the pruning table at the max depth, and search for the cheaper distances
    let mut tab: Table = Table { data: [DEPTH; SIZE] };

    // The first parent, is the solved cube
    let mut parents: VecDeque<Cube> = VecDeque::from(vec![Cube::new(); 1]);

    // Which has a distance of zero
    tab.data[0] = 0;

    // Apply 6 breadth first searches by applying turns to the cube, and recording the depth from the solved state, in
    // the pruning table.
    for depth in 1..DEPTH {
        parents = bfs(&mut tab, depth, &mut parents);
    }

    tab
}

/// bfs - TODO
fn bfs(tab: &mut Table, depth: usize, parents: &mut VecDeque<Cube>) -> VecDeque<Cube> {
    // Retrieve the number of valid moves in this group
    let valid = Group::Zero.moves().len();

    // Preallocate enough space for all the possible turns, in G0 there's valid turns per-level
    let mut children: VecDeque<Cube> = VecDeque::with_capacity(valid.pow(depth as u32));

    // Go through all the parent cubes, and visit them
    while let Some(cube) = parents.pop_front() {
        visit(depth, cube, tab, &mut children)
    }

    children
}

/// visit - TODO
fn visit(depth: usize, cube: Cube, tab: &mut Table, children: &mut VecDeque<Cube>) {
    for mv in Group::Zero.moves() {
        let mut cube = cube;

        cube.rotate(*mv);

        let i = idx(cube.edge_orientations());

        // Only update the pruning table, if we've found a shorter path
        if i != 0 && tab.data[i] > depth {
            tab.data[i] = depth;
        }

        children.push_front(cube);
    }
}

/// idx - TODO
fn idx(eorien: &[isize; NUM_EDGES]) -> usize {
    let mut dec = 0;

    for i in 0..NUM_EDGES - 1 {
        dec += eorien[i] as usize * (2 as usize).pow(10 - i as u32)
    }

    dec
}
