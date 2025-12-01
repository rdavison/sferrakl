use crate::key::{Code, Id, Key, KeyMap, Map};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Src {
    Ansi,
    Iso,
    Jis,
}

impl std::fmt::Display for Src {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Src::Ansi => write!(f, "Ansi"),
            Src::Iso => write!(f, "Iso"),
            Src::Jis => write!(f, "Jis"),
        }
    }
}

pub enum Layout {
    Ansi,
    Dvorak,
    Colemak,
    Custom(HashMap<Id, Code>),
}

#[rustfmt::skip]
pub const ANSI: [Id; 31] = {
    use Id::*;
    [
        Q, W, E, R, T, Y, U, I, O, P,
         A, S, D, F, G, H, J, K, L, Semi, Quot,
          Z, X, C, V, B, N, M, Comm, Prd, Slsh,
    ]
};

impl Src {
    pub fn keymap(&self) -> KeyMap<Key> {
        let ids = match self {
            Src::Ansi => ANSI.to_vec(),
            Src::Iso => todo!("Not Supported Yet: ISO Keyboard Src"),
            Src::Jis => todo!("Not Supported Yet: JIS Keyboard Src"),
        };
        let map = Map(ids
            .iter()
            .map(|id| {
                let key = id.key();
                (key.id, key)
            })
            .collect());
        KeyMap { src: *self, map }
    }
    pub fn keyboard(&self) -> Keyboard {
        let keymap = self.keymap();
        Keyboard::new(&keymap)
    }
}

pub struct Keyboard {
    keys: HashMap<Id, Key>,
}

impl Keyboard {
    pub fn new(keymap: &KeyMap<Key>) -> Self {
        Self {
            keys: keymap.map.0.clone(),
        }
    }

    pub fn nstrokes(&self, n: usize) -> NStrokeIterator {
        NStrokeIterator::new(self.keys.values().cloned().collect(), n)
    }

    pub fn set_layout(&mut self, layout: &Layout) {
        match layout {
            Layout::Ansi => {
                for key in self.keys.values_mut() {
                    key.code = key.id.to_code();
                }
            }
            Layout::Custom(custom_map) => {
                for (id, code) in custom_map {
                    if let Some(key) = self.keys.get_mut(id) {
                        key.code = Some(*code);
                    }
                }
            }
            _ => {
                // For full layouts like Dvorak, Colemak
                // First, reset all printable chars to the default Ansi/Qwerty
                for key in self.keys.values_mut() {
                    key.code = key.id.to_code();
                }

                // then apply the new layout on top of QWERTY
                let mapping = layout.mapping();
                for (id, code) in &mapping {
                    if let Some(key) = self.keys.get_mut(id) {
                        key.code = Some(*code);
                    }
                }
            }
        }
    }
}

impl Layout {
    pub fn mapping(&self) -> HashMap<Id, Code> {
        use Id::*;
        match self {
            Layout::Ansi => HashMap::new(), // Ansi (Qwerty) is baseline
            Layout::Dvorak => HashMap::from([
                (Q, Code::from('\'')),
                (W, Code::from(',')),
                (E, Code::from('.')),
                (R, Code::from('p')),
                (T, Code::from('y')),
                (Y, Code::from('f')),
                (U, Code::from('g')),
                (I, Code::from('c')),
                (O, Code::from('r')),
                (P, Code::from('l')),
                (S, Code::from('o')),
                (D, Code::from('e')),
                (F, Code::from('u')),
                (G, Code::from('i')),
                (H, Code::from('d')),
                (J, Code::from('h')),
                (K, Code::from('t')),
                (L, Code::from('n')),
                (Semi, Code::from('s')),
                (Quot, Code::from('-')),
                (Z, Code::from(';')),
                (X, Code::from('q')),
                (C, Code::from('j')),
                (V, Code::from('k')),
                (B, Code::from('x')),
                (N, Code::from('b')),
                (Comm, Code::from('w')),
                (Prd, Code::from('v')),
                (Slsh, Code::from('z')),
            ]),
            Layout::Colemak => HashMap::from([
                (E, Code::from('f')),
                (R, Code::from('p')),
                (T, Code::from('g')),
                (Y, Code::from('j')),
                (U, Code::from('l')),
                (I, Code::from('u')),
                (O, Code::from('y')),
                (P, Code::from(';')),
                (S, Code::from('r')),
                (D, Code::from('s')),
                (F, Code::from('t')),
                (G, Code::from('d')),
                (J, Code::from('n')),
                (K, Code::from('e')),
                (L, Code::from('i')),
                (Semi, Code::from('o')),
                (N, Code::from('k')),
            ]),
            Layout::Custom(map) => map.clone(),
        }
    }
}

#[derive(Clone)]
pub struct NStrokeIterator {
    keys: Vec<Key>,
    n: usize,
    indices: Vec<usize>,
    done: bool,
}

impl NStrokeIterator {
    fn new(keys: Vec<Key>, n: usize) -> Self {
        Self {
            keys,
            n,
            indices: vec![0; n],
            done: n == 0,
        }
    }
}

impl Iterator for NStrokeIterator {
    type Item = Vec<Key>;

    /*
      The iterator's state is stored in self.indices, a vector of n numbers that act as
      pointers to the keys vector.

       1. Termination Check: The function first checks a boolean flag self.done. If it's
          true, the iterator has exhausted all combinations and returns None, signaling
          the end.

       2. Construct Result: It creates the current n-stroke (result) by using the numbers
          in self.indices to pick the corresponding keys from the self.keys vector.

       3. Increment Logic: This is the core of the function. It works like counting on an
          odometer, starting from the rightmost digit (the last key in the stroke).
           * It tries to increment the rightmost index: self.indices[i] += 1.

       4. Carry-Over Check:
           * If the incremented index is still valid (i.e., less than the number of keys),
             the "odometer" has successfully ticked forward. The iterator's state is now
             set for the next stroke, so it returns the result it prepared in step 2.
           * If the index is out of bounds, it means that "digit" has rolled over. It is
             reset to 0, and the loop continues to the next index to the left, effectively
             "carrying over" the increment.

       5. Final State: If the while loop finishes, it means every index has rolled over
          (e.g., from [9, 9, 9] to [0, 0, 0]). This signifies that every possible
          combination has been produced. The self.done flag is set to true, and the very
          last combination is returned. On the subsequent call to next(), the function
          will return None.

      Example

      If you have 2 keys (k1, k2) and n=2, the indices vector will progress as follows on
      each call to next():
       - [0, 0] -> yields [k1, k1]
       - [0, 1] -> yields [k1, k2]
       - [1, 0] -> yields [k2, k1]
       - [1, 1] -> yields [k2, k2]
       - After this, self.done becomes true, and the next call returns None.
    */
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result: Vec<Key> = self.indices.iter().map(|&i| self.keys[i].clone()).collect();

        // Increment indices
        let mut i = self.n;
        while i > 0 {
            i -= 1;
            self.indices[i] += 1;
            if self.indices[i] < self.keys.len() {
                // Not done yet
                return Some(result);
            }
            // This dimension is done, reset and carry over
            self.indices[i] = 0;
        }

        // If we are here, all dimensions have been reset, so we are done
        self.done = true;
        Some(result)
    }
}
