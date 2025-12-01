use std::collections::HashMap;
use std::fs;
use std::io;
use std::marker::PhantomData;
use std::ops::AddAssign;

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Corpus<T>
where
    T: AddAssign + From<u8> + Copy + Default,
{
    pub ab: HashMap<String, T>, // distinct-2

    pub abc: HashMap<String, T>, // distinct-3
    pub a_b: HashMap<String, T>, // skip-1-distinct-2
    pub aba: HashMap<String, T>, // repeat-1-distinct-2
    pub cvc: HashMap<String, T>, // consonants-and-[aeiouy]
    pub vcv: HashMap<String, T>, // consonants-and-[aeiouy]

    pub abcd: HashMap<String, T>, // distinct-4
    pub a__b: HashMap<String, T>, // skip-2-distinct-2
    pub abab: HashMap<String, T>, // cycle-a-distinct-2
    pub abba: HashMap<String, T>, // cycle-b-distinct-2
    pub abca: HashMap<String, T>, // cycle-a-distinct-3
    pub abcb: HashMap<String, T>, // cycle-b-distinct-3
    pub cvvc: HashMap<String, T>, // consonants-and-[aeiouy]
    pub vccv: HashMap<String, T>, // consonants-and-[aeiouy]

    pub abcde: HashMap<String, T>, // distinct-4
    pub a___b: HashMap<String, T>, // skip-2-distinct-2
    pub cvvvc: HashMap<String, T>, // consonants-and-[aeiouy]
    pub ccvcc: HashMap<String, T>, // consonants-and-[aeiouy]
    pub vcccv: HashMap<String, T>, // consonants-and-[aeiouy]
    pub vvcvv: HashMap<String, T>, // consonants-and-[aeiouy]
    _phantom: PhantomData<T>,
}

impl<T> Corpus<T>
where
    T: AddAssign + From<u8> + Copy + Default,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            ..Self::default()
        }
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
                    *self.ab.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
            }
        }

        if len >= 3 {
            for w in chars.windows(3) {
                // abc
                if w[0] != w[1] && w[0] != w[2] && w[1] != w[2] {
                    *self.abc.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // a_b
                if w[0] != w[2] {
                    let s = format!("{}{}", w[0], w[2]);
                    *self.a_b.entry(s).or_insert(0.into()) += 1.into();
                }
                // aba
                if w[0] == w[2] && w[0] != w[1] {
                    *self.aba.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // cvc
                if !is_vowel(w[0]) && is_vowel(w[1]) && !is_vowel(w[2]) {
                    *self.cvc.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // vcv
                if is_vowel(w[0]) && !is_vowel(w[1]) && is_vowel(w[2]) {
                    *self.vcv.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
            }
        }

        if len >= 4 {
            for w in chars.windows(4) {
                // abcd
                let s: std::collections::HashSet<char> = w.iter().cloned().collect();
                if s.len() == 4 {
                    *self.abcd.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // a__b
                if w[0] != w[3] {
                    let s = format!("{}{}", w[0], w[3]);
                    *self.a__b.entry(s).or_insert(0.into()) += 1.into();
                }
                // abab
                if w[0] == w[2] && w[1] == w[3] && w[0] != w[1] {
                    *self.abab.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // abba
                if w[0] == w[3] && w[1] == w[2] && w[0] != w[1] {
                    *self.abba.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // abca
                if w[0] == w[3] && (w[0] != w[1] && w[0] != w[2] && w[1] != w[2]) {
                    *self.abca.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // abcb
                if w[1] == w[3] && (w[0] != w[1] && w[0] != w[2] && w[1] != w[2]) {
                    *self.abcb.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // cvvc
                if !is_vowel(w[0]) && is_vowel(w[1]) && is_vowel(w[2]) && !is_vowel(w[3]) {
                    *self.cvvc.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // vccv
                if is_vowel(w[0]) && !is_vowel(w[1]) && !is_vowel(w[2]) && is_vowel(w[3]) {
                    *self.vccv.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
            }
        }

        if len >= 5 {
            for w in chars.windows(5) {
                // abcde
                let s: std::collections::HashSet<char> = w.iter().cloned().collect();
                if s.len() == 5 {
                    *self.abcde.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // a___b
                if w[0] != w[4] {
                    let s = format!("{}{}", w[0], w[4]);
                    *self.a___b.entry(s).or_insert(0.into()) += 1.into();
                }
                // cvvvc
                if !is_vowel(w[0])
                    && is_vowel(w[1])
                    && is_vowel(w[2])
                    && is_vowel(w[3])
                    && !is_vowel(w[4])
                {
                    *self.cvvvc.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // ccvcc
                if !is_vowel(w[0])
                    && !is_vowel(w[1])
                    && is_vowel(w[2])
                    && !is_vowel(w[3])
                    && !is_vowel(w[4])
                {
                    *self.ccvcc.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // vcccv
                if is_vowel(w[0])
                    && !is_vowel(w[1])
                    && !is_vowel(w[2])
                    && !is_vowel(w[3])
                    && is_vowel(w[4])
                {
                    *self.vcccv.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
                // vvcvv
                if is_vowel(w[0])
                    && is_vowel(w[1])
                    && !is_vowel(w[2])
                    && is_vowel(w[3])
                    && is_vowel(w[4])
                {
                    *self.vvcvv.entry(w.iter().collect()).or_insert(0.into()) += 1.into();
                }
            }
        }
    }
}

pub fn from_file(path: &str) -> io::Result<Corpus<u64>> {
    let contents = fs::read_to_string(path)?;
    let mut corpus = Corpus::new();
    corpus.process_text(&contents);
    Ok(corpus)
}
