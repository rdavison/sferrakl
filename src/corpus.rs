use itertools::Itertools;
use std::{collections::HashMap, error::Error, fs, path::PathBuf, str::Chars};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Corpus {
    pub a: HashMap<String, u64>,
    pub ab: HashMap<String, u64>,
    pub abc: HashMap<String, u64>,
    pub a_b: HashMap<String, u64>,
    pub aba: HashMap<String, u64>,
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

pub fn of_path(path: PathBuf) -> Result<Corpus, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let versioned: VersionedCorpus = rmp_serde::from_slice(&bytes)?;
    let corpus = match versioned {
        VersionedCorpus::V1(c) => c,
    };
    Ok(corpus)
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
            *self.a.entry(a.to_string()).or_default() += 1;
            *self.ab.entry(format!("{}{}", a, b)).or_default() += 1;
            *self.abc.entry(format!("{}{}{}", a, b, c)).or_default() += 1;
            *self.a_b.entry(format!("{}{}", a, c)).or_default() += 1;
            if a == c {
                *self.aba.entry(format!("{}{}", a, b)).or_default() += 1;
            }
        }
    }

    pub fn save(self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let serialized = rmp_serde::to_vec(&VersionedCorpus::V1(self))?;
        fs::write(path, serialized)?;
        Ok(())
    }
}
