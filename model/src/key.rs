use super::finger::Finger;
use super::hand::Hand;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Id {
    Esc,  F1,   F2,   F3,   F4,   F5,  F6,   F7,   F8,   F9,   F10,  F11,  F12, Powr,
    Grav, _1,   _2,   _3,   _4,   _5,  _6,   _7,   _8,   _9,   _0,   Hyph, Eq, Bspc,
    Tab,  Q,    W,    E,    R,    T,   Y,    U,    I,    O,    P,    Obrk, Cbrk, Bsl,
    Caps, A,    S,    D,    F,    G,   H,    J,    K,    L,    Semi, Quot, Entr,
    Lsft, Z,    X,    C,    V,    B,   N,    M,    Comm, Prd,  Slsh, Rsft,
          Fn,   Lctl, Lopt, Lcmd, Spc, Rcmd, Ropt, Left, Down, Up,   Rght,
}

#[rustfmt::skip]
pub const ANSI30: [Id; 30] = {
    use Id::*;
    [
        Q,    W,    E,    R,    T,    Y,    U,    I,    O,    P,
        A,    S,    D,    F,    G,    H,    J,    K,    L,    Semi,
        Z,    X,    C,    V,    B,    N,    M,    Comm, Prd,  Slsh,
    ]
};

#[derive(Debug, Copy, Clone)]
pub enum Src {
    Ansi30,
}

impl std::fmt::Display for Src {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Src::Ansi30 => write!(f, "Ansi30"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Code(char);

impl std::fmt::Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Code").field(&self.0).finish()
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
            | V    | B    | N    | M => Finger::I,
            // Thumb (T)
            Fn   | Lctl | Lopt | Lcmd | Spc  | Rcmd | Ropt => Finger::T,
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
pub struct Map<T>(HashMap<Id, T>);

#[derive(Debug)]
pub struct KeyMap<T> {
    src: Src,
    map: Map<T>,
}

impl<T> Display for KeyMap<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(80);
        out.push_str(&format!("Src: {}", self.src));

        for (i, &id) in ANSI30.iter().enumerate() {
            if (i) % 10 == 0 {
                out.push('\n');
            }
            if let Some(code) = self.map.0.get(&id) {
                out.push_str(&format!("{} ", code));
            }
        }

        let out = out
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", out)
    }
}

pub struct Key {
    pub hand: super::hand::Hand,
    pub finger: super::finger::Finger,
    pub row: u8,
    pub code: Option<Code>,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
            && self.finger == other.finger
            && self.row == other.row
            && self.code == other.code
    }
}

impl Eq for Key {}

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hand.hash(state);
        self.finger.hash(state);
        self.row.hash(state);
        self.code.hash(state);
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
        Key {
            hand,
            finger,
            row,
            code,
        }
    }
}

impl Src {
    pub fn keymap(self) -> KeyMap<Key> {
        let mut map = HashMap::with_capacity(ANSI30.len());
        let map = Map(match self {
            Src::Ansi30 => {
                for &id in ANSI30.iter() {
                    let key = id.key();
                    map.insert(id, key);
                }
                map
            }
        });
        KeyMap { src: self, map }
    }
}

impl KeyMap<Key> {
    pub fn to_hand_finger_map(self) -> HashMap<Hand, HashMap<Finger, HashSet<Key>>> {
        let mut hand_finger_map: HashMap<Hand, HashMap<Finger, HashSet<Key>>> = HashMap::new();

        for (_, key) in self.map.0 {
            hand_finger_map
                .entry(key.hand)
                .or_default()
                .entry(key.finger)
                .or_default()
                .insert(key);
        }

        hand_finger_map
    }

    pub fn to_hand_finger_tuple_map(self) -> HashMap<(Hand, Finger), HashSet<Key>> {
        let mut hand_finger_map: HashMap<(Hand, Finger), HashSet<Key>> = HashMap::new();

        for (_, key) in self.map.0 {
            hand_finger_map
                .entry((key.hand, key.finger))
                .or_default()
                .insert(key);
        }

        hand_finger_map
    }
}
