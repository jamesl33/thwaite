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

    let mut tab: Table = Table { data: [DEPTH; SIZE] };

    let mut parents: VecDeque<Cube> = VecDeque::from(vec![Cube::new(); 1]);

    // TODO
    tab.data[0] = 0;

    for depth in 1..DEPTH {
        parents = dfs(&mut tab, depth, &mut parents);
    }

    tab
}

/// dfs - TODO
fn dfs(tab: &mut Table, depth: usize, parents: &mut VecDeque<Cube>) -> VecDeque<Cube> {
    let mut children: VecDeque<Cube> = VecDeque::with_capacity((18 as usize).pow(depth as u32));

    while let Some(cube) = parents.pop_front() {
        visit(depth, cube, tab, &mut children)
    }

    children
}

/// visit - TODO
fn visit(depth: usize, cube: Cube, tab: &mut Table, children: &mut VecDeque<Cube>) {
    for mv in Group::Zero.moves() {
        let mut cube = cube;

        cube.rotate(mv);

        let i = idx(cube.edge_orientations());

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
