use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Hand {
    L, // Left
    R, // Right
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand_char = match self {
            Hand::L => 'L',
            Hand::R => 'R',
        };
        write!(f, "{}", hand_char)
    }
}
