use crate::cube::{Color, Column, Face, Facelet, Row, NUM_EDGES};
use std::collections::HashMap;

/// The first edge, in the UL position.
const E0: Edge = Edge {
    a: Facelet::new(Face::Up, Row::Middle, Column::Left),
    b: Facelet::new(Face::Left, Row::Up, Column::Middle),
};

/// The second edge, in the DL position.
const E1: Edge = Edge {
    a: Facelet::new(Face::Left, Row::Down, Column::Middle),
    b: Facelet::new(Face::Down, Row::Middle, Column::Left),
};

/// The third edge, in the UR position.
const E2: Edge = Edge {
    a: Facelet::new(Face::Up, Row::Middle, Column::Right),
    b: Facelet::new(Face::Right, Row::Up, Column::Middle),
};

/// The fourth edge, in the DR position.
const E3: Edge = Edge {
    a: Facelet::new(Face::Right, Row::Down, Column::Middle),
    b: Facelet::new(Face::Down, Row::Middle, Column::Right),
};

/// The fifth edge, in the BL position.
const E4: Edge = Edge {
    a: Facelet::new(Face::Left, Row::Middle, Column::Left),
    b: Facelet::new(Face::Back, Row::Middle, Column::Right),
};

/// The sixth edge, in the FL position.
const E5: Edge = Edge {
    a: Facelet::new(Face::Front, Row::Middle, Column::Left),
    b: Facelet::new(Face::Left, Row::Middle, Column::Right),
};

/// The seventh edge, in the FR position.
const E6: Edge = Edge {
    a: Facelet::new(Face::Front, Row::Middle, Column::Right),
    b: Facelet::new(Face::Right, Row::Middle, Column::Left),
};

/// The eighth edge, in the BR position.
const E7: Edge = Edge {
    a: Facelet::new(Face::Back, Row::Middle, Column::Left),
    b: Facelet::new(Face::Right, Row::Middle, Column::Right),
};

/// The ninth edge, in the UF position.
const E8: Edge = Edge {
    a: Facelet::new(Face::Front, Row::Up, Column::Middle),
    b: Facelet::new(Face::Up, Row::Down, Column::Middle),
};

/// The tenth edge, in the DF position.
const E9: Edge = Edge {
    a: Facelet::new(Face::Front, Row::Down, Column::Middle),
    b: Facelet::new(Face::Down, Row::Up, Column::Middle),
};

/// The eleventh edge, in the DB position.
const E10: Edge = Edge {
    a: Facelet::new(Face::Back, Row::Down, Column::Middle),
    b: Facelet::new(Face::Down, Row::Down, Column::Middle),
};

/// The twelfth edge, in the UB position.
const E11: Edge = Edge {
    a: Facelet::new(Face::Up, Row::Up, Column::Middle),
    b: Facelet::new(Face::Back, Row::Up, Column::Middle),
};

/// The edge positions on the cube.
pub const EDGES: [Edge; NUM_EDGES] = [E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11];

/// Represents an edge piece on a cube, note this is positional information; the color isn't stored.
///
/// NOTE: We don't store as x, y or z because it's dependent ant on the permutation/orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    /// The first facelet.
    pub a: Facelet,

    /// The second facelet.
    pub b: Facelet,
}

impl Edge {
    /// Returns the edge for the given color pair.
    pub fn new(a: Color, b: Color) -> Self {
        let key = |a, b| {
            if a < b {
                return format!("{:?}:{:?}", a, b);
            }

            return format!("{:?}:{:?}", b, a);
        };

        let c2e = HashMap::from([
            (key(Color::Yellow, Color::Orange), E0),
            (key(Color::White, Color::Orange), E1),
            (key(Color::Yellow, Color::Red), E2),
            (key(Color::Red, Color::White), E3),
            (key(Color::Orange, Color::Green), E4),
            (key(Color::Blue, Color::Orange), E5),
            (key(Color::Red, Color::Blue), E6),
            (key(Color::Red, Color::Green), E7),
            (key(Color::Yellow, Color::Blue), E8),
            (key(Color::Blue, Color::White), E9),
            (key(Color::White, Color::Green), E10),
            (key(Color::Yellow, Color::Green), E11),
        ]);

        c2e.get(&key(a, b)).unwrap().to_owned()
    }

    /// Returns the identifier for the current piece (i.e. its location).
    pub fn id(self: &Self) -> usize {
        EDGES.iter().position(|e| e == self).unwrap()
    }
}
