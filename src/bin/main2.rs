use std::collections::HashMap;

use itertools::Itertools;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Corpus {
    ngrams1: HashMap<String, u64>,
    ngrams2: HashMap<String, u64>,
    ngrams3: HashMap<String, u64>,
}

fn main() {
    let s = std::fs::read_to_string("corpus.txt").unwrap();

    let mut ngrams1: HashMap<String, u64> = HashMap::new();
    let mut ngrams2: HashMap<String, u64> = HashMap::new();
    let mut ngrams3: HashMap<String, u64> = HashMap::new();

    for (a, b, c) in s.chars().tuple_windows() {
        *ngrams1.entry(a.to_string()).or_default() += 1;
        *ngrams2.entry(format!("{}{}", a, b)).or_default() += 1;
        *ngrams3.entry(format!("{}{}{}", a, b, c)).or_default() += 1;
    }

    let corpus = Corpus {
        ngrams1,
        ngrams2,
        ngrams3,
    };

    let serialized = serde_json::to_string(&corpus).unwrap();

    print!("{}", serialized);
}
