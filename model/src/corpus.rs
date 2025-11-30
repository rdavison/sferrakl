use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Corpus {
    pub ab: HashMap<String, u64>,     // distinct-2
    pub aba: HashMap<String, u64>,    // repeat-1-distinct-2
    pub abc: HashMap<String, u64>,    // distinct-3
    pub a_b: HashMap<String, u64>,    // skip-1-distinct-2
    #[allow(non_snake_case)]
    pub a__b: HashMap<String, u64>,   // skip-2-distinct-2
    pub abcab: HashMap<String, u64>,  // repeat-2-distinct-3
    pub abcba: HashMap<String, u64>,  // mirror-2-distinct-3
    pub abcdab: HashMap<String, u64>, // repeat-2-distinct-4
    pub abcdba: HashMap<String, u64>, // mirror-2-distinct-4
    pub ab_ab: HashMap<String, u64>,  // skip-1-repeat-2-distinct-2
    pub ab_ba: HashMap<String, u64>,  // skip-1-mirror-2-distinct-2
    #[allow(non_snake_case)]
    pub ab__ab: HashMap<String, u64>, // skip-2-repeat-2-distinct-2
    #[allow(non_snake_case)]
    pub ab__ba: HashMap<String, u64>, // skip-2-mirror-2-distinct-2
}

impl Corpus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_text(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();

        if len >= 2 {
            for w in chars.windows(2) {
                // ab: distinct-2
                if w[0] != w[1] {
                    let s = w.iter().collect::<String>();
                    *self.ab.entry(s).or_insert(0) += 1;
                }
            }
        }

        if len >= 3 {
            for w in chars.windows(3) {
                // aba: repeat-1-distinct-2
                if w[0] == w[2] && w[0] != w[1] {
                    let s = w.iter().collect::<String>();
                    *self.aba.entry(s).or_insert(0) += 1;
                }

                // abc: distinct-3
                if w[0] != w[1] && w[1] != w[2] && w[0] != w[2] {
                    let s = w.iter().collect::<String>();
                    *self.abc.entry(s).or_insert(0) += 1;
                }

                // a_b: skip-1-distinct-2
                if w[0] != w[2] {
                    let s = format!("{}{}", w[0], w[2]);
                    *self.a_b.entry(s).or_insert(0) += 1;
                }
            }
        }

        if len >= 4 {
            for w in chars.windows(4) {
                // a__b: skip-2-distinct-2
                if w[0] != w[3] {
                    let s = format!("{}{}", w[0], w[3]);
                    *self.a__b.entry(s).or_insert(0) += 1;
                }
            }
        }

        if len >= 5 {
            for w in chars.windows(5) {
                let are_distinct_3 = w[0] != w[1] && w[1] != w[2] && w[0] != w[2];
                let are_distinct_2 = w[0] != w[1];

                // abcab: repeat-2-distinct-3
                if w[0] == w[3] && w[1] == w[4] && are_distinct_3 {
                    let s = w.iter().collect::<String>();
                    *self.abcab.entry(s).or_insert(0) += 1;
                }

                // abcba: mirror-2-distinct-3
                if w[0] == w[4] && w[1] == w[3] && are_distinct_3 {
                    let s = w.iter().collect::<String>();
                    *self.abcba.entry(s).or_insert(0) += 1;
                }

                // ab_ab: skip-1-repeat-2-distinct-2
                if w[0] == w[3] && w[1] == w[4] && are_distinct_2 {
                    let s = w.iter().collect::<String>();
                    *self.ab_ab.entry(s).or_insert(0) += 1;
                }

                // ab_ba: skip-1-mirror-2-distinct-2
                if w[0] == w[4] && w[1] == w[3] && are_distinct_2 {
                    let s = w.iter().collect::<String>();
                    *self.ab_ba.entry(s).or_insert(0) += 1;
                }
            }
        }

        if len >= 6 {
            for w in chars.windows(6) {
                let are_distinct_4: bool = {
                    let mut s = std::collections::HashSet::new();
                    s.insert(w[0]);
                    s.insert(w[1]);
                    s.insert(w[2]);
                    s.insert(w[3]);
                    s.len() == 4
                };
                let are_distinct_2 = w[0] != w[1];

                // abcdab: repeat-2-distinct-4
                if w[0] == w[4] && w[1] == w[5] && are_distinct_4 {
                    let s = w.iter().collect::<String>();
                    *self.abcdab.entry(s).or_insert(0) += 1;
                }

                // abcdba: mirror-2-distinct-4
                if w[0] == w[5] && w[1] == w[4] && are_distinct_4 {
                    let s = w.iter().collect::<String>();
                    *self.abcdba.entry(s).or_insert(0) += 1;
                }

                // ab__ab: skip-2-repeat-2-distinct-2
                if w[0] == w[4] && w[1] == w[5] && are_distinct_2 {
                    let s = w.iter().collect::<String>();
                    *self.ab__ab.entry(s).or_insert(0) += 1;
                }

                // ab__ba: skip-2-mirror-2-distinct-2
                if w[0] == w[5] && w[1] == w[4] && are_distinct_2 {
                    let s = w.iter().collect::<String>();
                    *self.ab__ba.entry(s).or_insert(0) += 1;
                }
            }
        }
    }
}

pub fn from_file(path: &str) -> io::Result<Corpus> {
    let contents = fs::read_to_string(path)?;
    let mut corpus = Corpus::new();
    corpus.process_text(&contents);
    Ok(corpus)
}
