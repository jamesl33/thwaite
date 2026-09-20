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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_pairs_opposite_faces_together() {
        assert_eq!(Face::Left.axis(), Axis::X);
        assert_eq!(Face::Right.axis(), Axis::X);
        assert_eq!(Face::Up.axis(), Axis::Y);
        assert_eq!(Face::Down.axis(), Axis::Y);
        assert_eq!(Face::Front.axis(), Axis::Z);
        assert_eq!(Face::Back.axis(), Axis::Z);
    }

    #[test]
    fn color_gives_every_face_a_distinct_solved_color() {
        let faces = [Face::Up, Face::Right, Face::Front, Face::Down, Face::Left, Face::Back];
        let colors: Vec<Color> = faces.iter().map(|f| f.color()).collect();

        for i in 0..colors.len() {
            for j in (i + 1)..colors.len() {
                assert_ne!(colors[i], colors[j], "{:?} and {:?} share a color", faces[i], faces[j]);
            }
        }
    }
}
