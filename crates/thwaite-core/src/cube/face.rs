use crate::cube::Axis;
use crate::cube::Color;

/// Represents the fact on a cube.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back,
}

impl Face {
    /// Returns the axis for the face.
    pub fn axis(self: &Self) -> Axis {
        match self {
            Face::Left | Face::Right => Axis::X,
            Face::Up | Face::Down => Axis::Y,
            Face::Front | Face::Back => Axis::Z,
        }
    }

    /// Returns the solved color, for this face (i.e. not the color that's actually there).
    pub fn color(self: &Self) -> Color {
        match self {
            Face::Up => Color::Yellow,
            Face::Right => Color::Red,
            Face::Front => Color::Blue,
            Face::Down => Color::White,
            Face::Left => Color::Orange,
            Face::Back => Color::Green,
        }
    }
}
