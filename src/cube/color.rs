/// Represents a cube color; we model the standard colors for the cube.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Color {
    Yellow = 0,
    Red,
    Blue,
    White,
    Orange,
    Green,
}

impl From<&u8> for Color {
    /// Parses a color from an ASCII character.
    fn from(c: &u8) -> Self {
        match c {
            b'y' | b'Y' => Color::Yellow,
            b'r' | b'R' => Color::Red,
            b'b' | b'B' => Color::Blue,
            b'w' | b'W' => Color::White,
            b'o' | b'O' => Color::Orange,
            b'g' | b'G' => Color::Green,
            _ => unimplemented!(),
        }
    }
}
