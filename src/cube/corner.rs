use crate::cube::{Axis, Color, Column, Face, Facelet, Row, NUM_CORNERS};
use std::collections::HashMap;

/// The first corner, in the ULB position.
const C0: Corner = Corner {
    a: Facelet::new(Face::Up, Row::Up, Column::Left),
    b: Facelet::new(Face::Left, Row::Up, Column::Left),
    c: Facelet::new(Face::Back, Row::Up, Column::Right),
};

/// The second corner, in the DLF position.
const C1: Corner = Corner {
    a: Facelet::new(Face::Front, Row::Down, Column::Left),
    b: Facelet::new(Face::Left, Row::Down, Column::Right),
    c: Facelet::new(Face::Down, Row::Up, Column::Left),
};

/// The third corner, in the DBR position.
const C2: Corner = Corner {
    a: Facelet::new(Face::Right, Row::Down, Column::Right),
    b: Facelet::new(Face::Back, Row::Down, Column::Left),
    c: Facelet::new(Face::Down, Row::Down, Column::Right),
};

/// The fourth corner, in the URF position.
const C3: Corner = Corner {
    a: Facelet::new(Face::Up, Row::Down, Column::Right),
    b: Facelet::new(Face::Front, Row::Up, Column::Right),
    c: Facelet::new(Face::Right, Row::Up, Column::Left),
};

/// The fifth corner, in the ULF position.
const C4: Corner = Corner {
    a: Facelet::new(Face::Up, Row::Down, Column::Left),
    b: Facelet::new(Face::Front, Row::Up, Column::Left),
    c: Facelet::new(Face::Left, Row::Up, Column::Right),
};

/// The sixth corner, in the DLB position.
const C5: Corner = Corner {
    a: Facelet::new(Face::Left, Row::Down, Column::Left),
    b: Facelet::new(Face::Back, Row::Down, Column::Right),
    c: Facelet::new(Face::Down, Row::Down, Column::Left),
};

/// The seventh corner, in the DFR position.
const C6: Corner = Corner {
    a: Facelet::new(Face::Front, Row::Down, Column::Right),
    b: Facelet::new(Face::Down, Row::Up, Column::Right),
    c: Facelet::new(Face::Right, Row::Down, Column::Left),
};

/// The eighth corner, in the UBR position.
const C7: Corner = Corner {
    a: Facelet::new(Face::Up, Row::Up, Column::Right),
    b: Facelet::new(Face::Right, Row::Up, Column::Right),
    c: Facelet::new(Face::Back, Row::Up, Column::Left),
};

/// The edge positions on the cube.
pub const CORNERS: [Corner; NUM_CORNERS] = [C0, C1, C2, C3, C4, C5, C6, C7];

/// Represents an corner piece on a cube, note this is positional information; the color isn't stored.
///
/// NOTE: We don't store as x, y or z because it's dependent ant on the permutation/orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corner {
    /// The first facelet.
    pub a: Facelet,

    /// The second facelet.
    pub b: Facelet,

    /// The third facelet.
    pub c: Facelet,
}

impl Corner {
    /// Returns the corner for the given color triplet.
    pub fn new(a: Color, b: Color, c: Color) -> Self {
        let key = |a, b, c| {
            let mut colors = vec![a, b, c];

            colors.sort();

            format!("{:?}:{:?}:{:?}", colors[0], colors[1], colors[2])
        };

        let c2c = HashMap::from([
            (key(Color::Yellow, Color::Orange, Color::Green), C0),
            (key(Color::White, Color::Orange, Color::Blue), C1),
            (key(Color::Red, Color::White, Color::Green), C2),
            (key(Color::Yellow, Color::Blue, Color::Red), C3),
            (key(Color::Orange, Color::Blue, Color::Yellow), C4),
            (key(Color::Orange, Color::White, Color::Green), C5),
            (key(Color::Blue, Color::Red, Color::White), C6),
            (key(Color::Yellow, Color::Green, Color::Red), C7),
        ]);

        c2c.get(&key(a, b, c)).unwrap().to_owned()
    }

    /// Returns the identifier for the current piece (i.e. its location).
    pub fn id(self: &Self) -> usize {
        CORNERS.iter().position(|c| c == self).unwrap()
    }

    /// Returns the facelets in the x, y and z order.
    pub fn axis(self: &Self) -> Vec<Facelet> {
        vec![self.facelet(Axis::X), self.facelet(Axis::Y), self.facelet(Axis::Z)]
    }

    /// Returns the facelet for the given axis.
    fn facelet(self: &Self, a: Axis) -> Facelet {
        if self.a.face.axis() == a {
            return self.a;
        }

        if self.b.face.axis() == a {
            return self.b;
        }

        if self.c.face.axis() == a {
            return self.c;
        }

        unreachable!()
    }
}
