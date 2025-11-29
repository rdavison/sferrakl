use crate::finger::Finger;
use crate::hand::Hand;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HandFinger(pub (Hand, Finger));

impl fmt::Display for HandFinger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandFinger((hand, finger)) => write!(f, "{}{}", hand, finger),
        }
    }
}
