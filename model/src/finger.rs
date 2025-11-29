use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Finger {
    P, // Pinky
    R, // Ring
    M, // Middle
    I, // Index
    T, // Thumb
}

impl fmt::Display for Finger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let finger_char = match self {
            Finger::P => 'P',
            Finger::R => 'R',
            Finger::M => 'M',
            Finger::I => 'I',
            Finger::T => 'T',
        };
        write!(f, "{}", finger_char)
    }
}
