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
            _ => panic!("invalid color character: {}", *c as char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_color_letter_case_insensitively() {
        assert_eq!(Color::from(&b'y'), Color::Yellow);
        assert_eq!(Color::from(&b'Y'), Color::Yellow);
        assert_eq!(Color::from(&b'r'), Color::Red);
        assert_eq!(Color::from(&b'R'), Color::Red);
        assert_eq!(Color::from(&b'b'), Color::Blue);
        assert_eq!(Color::from(&b'B'), Color::Blue);
        assert_eq!(Color::from(&b'w'), Color::White);
        assert_eq!(Color::from(&b'W'), Color::White);
        assert_eq!(Color::from(&b'o'), Color::Orange);
        assert_eq!(Color::from(&b'O'), Color::Orange);
        assert_eq!(Color::from(&b'g'), Color::Green);
        assert_eq!(Color::from(&b'G'), Color::Green);
    }

    #[test]
    #[should_panic(expected = "invalid color character: x")]
    fn parsing_rejects_an_unknown_letter() {
        let _ = Color::from(&b'x');
    }
}
