use itertools::Itertools;
use std::{collections::HashMap, error::Error, fs, path::PathBuf, str::Chars};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Corpus {
    pub a: HashMap<char, u64>,
    pub ab: HashMap<(char, char), u64>,
    pub abc: HashMap<(char, char, char), u64>,
    pub a_b: HashMap<(char, char), u64>,
    pub aba: HashMap<(char, char), u64>,
}

#[derive(Serialize, Deserialize)]
pub enum VersionedCorpus {
    V1(Corpus),
}

pub fn of_string(s: &str) -> Corpus {
    let mut corpus: Corpus = Default::default();
    corpus.read(&mut s.chars());
    corpus
}

impl Default for Corpus {
    #[inline]
    fn default() -> Corpus {
        Corpus {
            a: HashMap::new(),
            ab: HashMap::new(),
            abc: HashMap::new(),
            a_b: HashMap::new(),
            aba: HashMap::new(),
        }
    }
}

impl Corpus {
    pub fn read(&mut self, iter: &mut Chars) {
        for (a, b, c) in iter.tuple_windows() {
            *self.a.entry(a).or_default() += 1;
            *self.ab.entry((a, b)).or_default() += 1;
            *self.abc.entry((a, b, c)).or_default() += 1;
            *self.a_b.entry((a, c)).or_default() += 1;
            if a == c {
                *self.aba.entry((a, b)).or_default() += 1;
            }
        }
    }

    pub fn write(self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let serialized = rmp_serde::to_vec(&VersionedCorpus::V1(self))?;
        fs::write(path, serialized)?;
        Ok(())
    }
}
