use std::collections::HashMap;

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

pub struct Code(char);

impl Code {
    pub fn value(&self) -> char {
        self.0
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
}

pub struct Map(HashMap<Id, Code>);

impl Src {
    pub fn keymap(self) -> Map {
        Map(match self {
            Src::Ansi30 => {
                let mut map = HashMap::with_capacity(ANSI30.len());

                for &id in ANSI30.iter() {
                    if let Some(code) = id.to_code() {
                        map.insert(id, code);
                    }
                }
                map
            }
        })
    }
}

pub struct Key {
    src: Src,
    map: Map,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key {{ src: {}, mappings: {} }}", self.src, self.map.0.len())
    }
}
