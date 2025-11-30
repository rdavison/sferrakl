use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Corpus {
    pub ab: HashMap<String, u64>, // distinct-2

    pub abc: HashMap<String, u64>, // distinct-3
    pub a_b: HashMap<String, u64>, // skip-1-distinct-2
    pub aba: HashMap<String, u64>, // repeat-1-distinct-2
    pub cvc: HashMap<String, u64>, // consonants-and-[aeiouy]
    pub vcv: HashMap<String, u64>, // consonants-and-[aeiouy]

    pub abcd: HashMap<String, u64>, // distinct-4
    pub a__b: HashMap<String, u64>, // skip-2-distinct-2
    pub abab: HashMap<String, u64>, // cycle-a-distinct-2
    pub abba: HashMap<String, u64>, // cycle-b-distinct-2
    pub abca: HashMap<String, u64>, // cycle-a-distinct-3
    pub abcb: HashMap<String, u64>, // cycle-b-distinct-3
    pub cvvc: HashMap<String, u64>, // consonants-and-[aeiouy]
    pub vccv: HashMap<String, u64>, // consonants-and-[aeiouy]

    pub abcde: HashMap<String, u64>, // distinct-4
    pub a___b: HashMap<String, u64>, // skip-2-distinct-2
    pub cvvvc: HashMap<String, u64>, // consonants-and-[aeiouy]
    pub ccvcc: HashMap<String, u64>, // consonants-and-[aeiouy]
    pub vcccv: HashMap<String, u64>, // consonants-and-[aeiouy]
    pub vvcvv: HashMap<String, u64>, // consonants-and-[aeiouy]
}

impl Corpus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_text(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();

        let is_vowel =
            |c: char| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y');

        if len >= 2 {
            for w in chars.windows(2) {
                // ab
                if w[0] != w[1] {
                    *self.ab.entry(w.iter().collect()).or_insert(0) += 1;
                }
            }
        }

        if len >= 3 {
            for w in chars.windows(3) {
                // abc
                if w[0] != w[1] && w[0] != w[2] && w[1] != w[2] {
                    *self.abc.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // a_b
                if w[0] != w[2] {
                    let s = format!("{}{}", w[0], w[2]);
                    *self.a_b.entry(s).or_insert(0) += 1;
                }
                // aba
                if w[0] == w[2] && w[0] != w[1] {
                    *self.aba.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // cvc
                if !is_vowel(w[0]) && is_vowel(w[1]) && !is_vowel(w[2]) {
                    *self.cvc.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // vcv
                if is_vowel(w[0]) && !is_vowel(w[1]) && is_vowel(w[2]) {
                    *self.vcv.entry(w.iter().collect()).or_insert(0) += 1;
                }
            }
        }

        if len >= 4 {
            for w in chars.windows(4) {
                // abcd
                let s: std::collections::HashSet<char> = w.iter().cloned().collect();
                if s.len() == 4 {
                    *self.abcd.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // a__b
                if w[0] != w[3] {
                    let s = format!("{}{}", w[0], w[3]);
                    *self.a__b.entry(s).or_insert(0) += 1;
                }
                // abab
                if w[0] == w[2] && w[1] == w[3] && w[0] != w[1] {
                    *self.abab.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // abba
                if w[0] == w[3] && w[1] == w[2] && w[0] != w[1] {
                    *self.abba.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // abca
                if w[0] == w[3] && (w[0] != w[1] && w[0] != w[2] && w[1] != w[2]) {
                    *self.abca.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // abcb
                if w[1] == w[3] && (w[0] != w[1] && w[0] != w[2] && w[1] != w[2]) {
                    *self.abcb.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // cvvc
                if !is_vowel(w[0]) && is_vowel(w[1]) && is_vowel(w[2]) && !is_vowel(w[3]) {
                    *self.cvvc.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // vccv
                if is_vowel(w[0]) && !is_vowel(w[1]) && !is_vowel(w[2]) && is_vowel(w[3]) {
                    *self.vccv.entry(w.iter().collect()).or_insert(0) += 1;
                }
            }
        }

        if len >= 5 {
            for w in chars.windows(5) {
                // abcde
                let s: std::collections::HashSet<char> = w.iter().cloned().collect();
                if s.len() == 5 {
                    *self.abcde.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // a___b
                if w[0] != w[4] {
                    let s = format!("{}{}", w[0], w[4]);
                    *self.a___b.entry(s).or_insert(0) += 1;
                }
                // cvvvc
                if !is_vowel(w[0])
                    && is_vowel(w[1])
                    && is_vowel(w[2])
                    && is_vowel(w[3])
                    && !is_vowel(w[4])
                {
                    *self.cvvvc.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // ccvcc
                if !is_vowel(w[0])
                    && !is_vowel(w[1])
                    && is_vowel(w[2])
                    && !is_vowel(w[3])
                    && !is_vowel(w[4])
                {
                    *self.ccvcc.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // vcccv
                if is_vowel(w[0])
                    && !is_vowel(w[1])
                    && !is_vowel(w[2])
                    && !is_vowel(w[3])
                    && is_vowel(w[4])
                {
                    *self.vcccv.entry(w.iter().collect()).or_insert(0) += 1;
                }
                // vvcvv
                if is_vowel(w[0])
                    && is_vowel(w[1])
                    && !is_vowel(w[2])
                    && is_vowel(w[3])
                    && is_vowel(w[4])
                {
                    *self.vvcvv.entry(w.iter().collect()).or_insert(0) += 1;
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
