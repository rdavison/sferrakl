use super::finger::Finger;
use super::hand::Hand;
use std::collections::HashMap;

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Id {
    Esc,  F1,   F2,   F3,   F4,   F5,  F6,   F7,   F8,   F9,   F10,  F11,  F12, Powr,
    Grav, _1,   _2,   _3,   _4,   _5,  _6,   _7,   _8,   _9,   _0,   Hyph, Eq, Bspc,
    Tab,  Q,    W,    E,    R,    T,   Y,    U,    I,    O,    P,    Obrk, Cbrk, Bsl,
    Caps, A,    S,    D,    F,    G,   H,    J,    K,    L,    Semi, Quot, Entr,
    Lsft, Z,    X,    C,    V,    B,   N,    M,    Comm, Prd,  Slsh, Rsft,
          Fn,   Lctl, Lopt, Lcmd, Spc, Rcmd, Ropt, Left, Down, Up,   Rght,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Code(char);

impl From<char> for Code {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl std::fmt::Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Code(c) = self;
        f.debug_tuple("Code").field(c).finish()
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Code(c) = self;
        write!(f, "{}", c)
    }
}

impl Id {
    pub fn to_code(self) -> Option<Code> {
        use self::Id::*;
        let char_opt = match self {
            // Letters
            Q => Some('q'),
            W => Some('w'),
            E => Some('e'),
            R => Some('r'),
            T => Some('t'),
            Y => Some('y'),
            U => Some('u'),
            I => Some('i'),
            O => Some('o'),
            P => Some('p'),
            A => Some('a'),
            S => Some('s'),
            D => Some('d'),
            F => Some('f'),
            G => Some('g'),
            H => Some('h'),
            J => Some('j'),
            K => Some('k'),
            L => Some('l'),
            Z => Some('z'),
            X => Some('x'),
            C => Some('c'),
            V => Some('v'),
            B => Some('b'),
            N => Some('n'),
            M => Some('m'),

            // Numbers
            _1 => Some('1'),
            _2 => Some('2'),
            _3 => Some('3'),
            _4 => Some('4'),
            _5 => Some('5'),
            _6 => Some('6'),
            _7 => Some('7'),
            _8 => Some('8'),
            _9 => Some('9'),
            _0 => Some('0'),

            // Symbols
            Grav => Some('`'),
            Hyph => Some('-'),
            Eq => Some('='),
            Obrk => Some('['),
            Cbrk => Some(']'),
            Bsl => Some('\\'),
            Semi => Some(';'),
            Quot => Some('\''),
            Comm => Some(','),
            Prd => Some('.'),
            Slsh => Some('/'),

            // Whitespace / Control Chars
            Spc => Some(' '),
            Tab => Some('\t'),
            Entr => Some('\n'),

            // Non-printable / Functional keys
            Esc | F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 | F9 | F10 | F11 | F12 | Powr | Bspc
            | Caps | Lsft | Rsft | Fn | Lctl | Lopt | Lcmd | Rcmd | Ropt | Left | Down | Up
            | Rght => None,
        };
        char_opt.map(Code)
    }

    #[rustfmt::skip]
    pub fn default_hand(self) -> Hand {
        use Id::*;
        match self {
            // Left hand
              Esc  | F1   | F2   | F3   | F4   | F5
            | Grav | _1   | _2   | _3   | _4   | _5
            | Tab  | Q    | W    | E    | R    | T
            | Caps | A    | S    | D    | F    | G
            | Lsft | Z    | X    | C    | V    | B
            | Fn   | Lctl | Lopt | Lcmd => Hand::L,
            // Right hand
              F6   | F7   | F8   | F9   | F10  | F11  | F12  | Powr
            | _6   | _7   | _8   | _9   | _0   | Hyph | Eq   | Bspc
            | Y    | U    | I    | O    | P    | Obrk | Cbrk | Bsl
            | H    | J    | K    | L    | Semi | Quot | Entr
            | N    | M    | Comm | Prd  | Slsh | Rsft
            | Spc  | Rcmd | Ropt
            | Left | Down | Up | Rght => Hand::R,
        }
    }

    #[rustfmt::skip]
    pub fn default_finger(self) -> Finger {
        use Id::*;
        match self {
            // Pinky (P)
              Esc  | F1  | F10  | F11  | F12  | Powr
            | Grav | _1  | _0   | Hyph | Eq   | Bspc
            | Tab  | Q   | P    | Obrk | Cbrk | Bsl
            | Caps | A   | Semi | Quot | Entr
            | Lsft | Z   | Slsh | Rsft => Finger::P,
            // Ring (R)
              F2   | F9
            | _2   | _9
            | W    | O
            | S    | L
            | X    | Prd => Finger::R,
            // Middle (M)
              F3   | F8
            | _3   | _8
            | E    | I
            | D    | K
            | C    | Comm => Finger::M,
            // Index (I)
              F4   | F5   | F6   | F7
            | _4   | _5   | _6   | _7
            | R    | T    | Y    | U
            | F    | G    | H    | J
            | V    | B | N | M => Finger::I,
            // Thumb (T)
            Fn | Lctl | Lopt | Lcmd | Spc | Rcmd | Ropt => Finger::T,
            // Arrows (mixed)
            Left => Finger::I,
            Rght => Finger::R,
            Up | Down => Finger::M,
        }
    }

    pub fn default_row(self) -> u8 {
        use Id::*;
        match self {
            // Function row
            Esc | F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 | F9 | F10 | F11 | F12 | Powr => 0,
            // Number row
            Grav | _1 | _2 | _3 | _4 | _5 | _6 | _7 | _8 | _9 | _0 | Hyph | Eq | Bspc => 1,
            // QWERTY row
            Tab | Q | W | E | R | T | Y | U | I | O | P | Obrk | Cbrk | Bsl => 2,
            // ASDFG row
            Caps | A | S | D | F | G | H | J | K | L | Semi | Quot | Entr => 3,
            // ZXCVB row
            Lsft | Z | X | C | V | B | N | M | Comm | Prd | Slsh | Rsft => 4,
            // Bottom row
            Fn | Lctl | Lopt | Lcmd | Spc | Rcmd | Ropt | Left | Down | Up | Rght => 5,
        }
    }
}

#[derive(Debug)]
pub struct Map<T>(pub HashMap<Id, T>);

#[derive(Clone, Debug)]
pub struct Key {
    pub id: Id,
    pub hand: super::hand::Hand,
    pub finger: super::finger::Finger,
    pub row: u8,
    pub code: Option<Code>,
    pub x: f32,
    pub y: f32,
}

impl Key {
    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.hand == other.hand
            && self.finger == other.finger
            && self.row == other.row
            && self.code == other.code
            && self.x.to_bits() == other.x.to_bits()
            && self.y.to_bits() == other.y.to_bits()
    }
}

impl Eq for Key {}

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.hand.hash(state);
        self.finger.hash(state);
        self.row.hash(state);
        self.code.hash(state);
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code {
            Some(code) => write!(f, "{}", code),
            None => write!(f, "_"), // Placeholder for non-printable keys
        }
    }
}

impl Id {
    pub fn key(self) -> Key {
        let hand = self.default_hand();
        let finger = self.default_finger();
        let row = self.default_row();
        let code = self.to_code();
        let (x, y) = self.default_coords();
        Key {
            id: self,
            hand,
            finger,
            row,
            code,
            x,
            y,
        }
    }

    #[rustfmt::skip]
    pub fn default_coords(self) -> (f32, f32) {
        use Id::*;
        match self {
            Esc  => (0.0, 0.0), F1 => (1.0, 0.0), F2 => (2.0, 0.0), F3 => (3.0, 0.0), F4 => (4.0, 0.0), F5 => (5.0, 0.0), F6 => (6.0, 0.0), F7 => (7.0, 0.0), F8 => (8.0, 0.0), F9 => (9.0, 0.0), F10 => (11.0, 0.0), F11 => (12.0, 0.0), F12 => (13.0, 0.0), Powr => (14.0, 0.0),
            Grav => (0.0, 1.0), _1 => (1.0, 1.0), _2 => (2.0, 1.0), _3 => (3.0, 1.0), _4 => (4.0, 1.0), _5 => (5.0, 1.0), _6 => (6.0, 1.0), _7 => (7.0, 1.0), _8 => (8.0, 1.0), _9 => (9.0, 1.0), _0 => (10.0, 1.0), Hyph => (11.0, 1.0), Eq => (12.0, 1.0), Bspc => (13.0, 1.0),
            Tab  => (0.0, 2.0), Q => (1.0, 2.0), W => (2.0, 2.0), E => (3.0, 2.0), R => (4.0, 2.0), T => (5.0, 2.0), Y => (6.0, 2.0), U => (7.0, 2.0), I => (8.0, 2.0), O => (9.0, 2.0), P => (10.0, 2.0), Obrk => (11.0, 2.0), Cbrk => (12.0, 2.0), Bsl => (13.0, 2.0),
            Caps => (0.0, 3.0), A => (1.0, 3.0), S => (2.0, 3.0), D => (3.0, 3.0), F => (4.0, 3.0), G => (5.0, 3.0), H => (6.0, 3.0), J => (7.0, 3.0), K => (8.0, 3.0), L => (9.0, 3.0), Semi => (10.0, 3.0), Quot => (11.0, 3.0), Entr => (12.0, 3.0),
            Lsft => (0.0, 4.0), Z => (1.0, 4.0), X => (2.0, 4.0), C => (3.0, 4.0), V => (4.0, 4.0), B => (5.0, 4.0), N => (6.0, 4.0), M => (7.0, 4.0), Comm => (8.0, 4.0), Prd => (9.0, 4.0), Slsh => (10.0, 4.0), Rsft => (11.0, 4.0),
            Fn => (0.0, 5.0), Lctl => (1.0, 5.0), Lopt => (2.0, 5.0), Lcmd => (3.0, 5.0), Spc => (4.0, 5.0), Rcmd => (5.0, 5.0), Ropt => (6.0, 5.0), Left => (7.0, 5.0), Down => (8.0, 5.0), Up => (9.0, 5.0), Rght => (10.0, 5.0),
        }
    }

    #[rustfmt::skip]
    pub fn alternate_fingers(self) -> Vec<Finger> {
        match self {
            // Left hand index finger alternates
            Id::F | Id::G | Id::V | Id::B => vec![Finger::M],
            // Left hand middle finger alternates
            Id::D | Id::C => vec![Finger::I],
            // Left hand ring finger alternates
            Id::S | Id::X => vec![Finger::M],

            // Right hand index finger alternates
            Id::J | Id::H | Id::M | Id::N => vec![Finger::M],
            // Right hand middle finger alternates
            Id::K | Id::Comm => vec![Finger::I],
            // Right hand ring finger alternates
            Id::L | Id::Prd => vec![Finger::M],

            _ => vec![],
        }
    }
}
