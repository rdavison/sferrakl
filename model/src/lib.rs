pub mod bigram;
pub mod corpus;
pub mod finger;
pub mod hand;
pub mod hand_finger;
pub mod key;
pub mod keyboard;
pub mod tier;

#[cfg(test)]
mod tests {

    use super::bigram;
    use super::finger::Finger;
    use super::hand::Hand;
    use super::hand_finger::HandFinger;
    use super::key::Src;
    use super::key::ANSI30;

    #[test]
    fn key_display() {
        let actual = Src::Ansi30.keymap().to_string();
        let expected = indoc::indoc! {"
            Src: Ansi30
            q w e r t y u i o p
            a s d f g h j k l ;
            z x c v b n m , . /
        "}
        .trim_end();
        assert_eq!(actual, expected);
    }

    #[test]
    fn bigram_init() {
        let bigram_map = bigram::Map::init(Src::Ansi30, &|_, _| 1.0f64);
        let bigram::Map(map) = bigram_map;
        assert_eq!(map.len(), ANSI30.len() * ANSI30.len());
    }

    #[test]
    fn finger_enum() {
        assert_eq!(format!("{}", Finger::P), "P");
        assert_eq!(format!("{}", Finger::R), "R");
        assert_eq!(format!("{}", Finger::M), "M");
        assert_eq!(format!("{}", Finger::I), "I");
        assert_eq!(format!("{}", Finger::T), "T");
    }

    #[test]
    fn hand_enum() {
        assert_eq!(format!("{}", Hand::L), "L");
        assert_eq!(format!("{}", Hand::R), "R");
    }

    #[test]
    fn hand_finger_display() {
        let hand_finger_l_p = HandFinger((Hand::L, Finger::P));
        assert_eq!(format!("{}", hand_finger_l_p), "LP");

        let hand_finger_r_i = HandFinger((Hand::R, Finger::I));
        assert_eq!(format!("{}", hand_finger_r_i), "RI");
    }

    #[test]
    fn test_to_hand_finger_map() {
        let key_map = Src::Ansi30.keymap();
        let hand_finger_map = key_map.to_hand_finger_map();

        // Check some specific keys
        let q_keys = hand_finger_map
            .get(&Hand::L)
            .unwrap()
            .get(&Finger::P)
            .unwrap();
        assert!(q_keys
            .iter()
            .any(|k| k.code.map(|c| c.to_string()) == Some("q".to_string())));

        let j_keys = hand_finger_map
            .get(&Hand::R)
            .unwrap()
            .get(&Finger::I)
            .unwrap();
        assert!(j_keys
            .iter()
            .any(|k| k.code.map(|c| c.to_string()) == Some("j".to_string())));
        assert!(j_keys
            .iter()
            .any(|k| k.code.map(|c| c.to_string()) == Some("u".to_string())));

        // Check total number of keys
        let mut total_keys = 0;
        for (_, finger_map) in hand_finger_map.iter() {
            for (_, keys_set) in finger_map.iter() {
                total_keys += keys_set.len();
            }
        }
        assert_eq!(total_keys, ANSI30.len());
    }

    #[test]
    fn test_to_hand_finger_tuple_map() {
        let key_map = Src::Ansi30.keymap();
        let hand_finger_map = key_map.to_hand_finger_tuple_map();

        // Check some specific keys
        let q_keys = hand_finger_map.get(&(Hand::L, Finger::P)).unwrap();
        assert!(q_keys
            .iter()
            .any(|k| k.code.map(|c| c.to_string()) == Some("q".to_string())));

        let j_keys = hand_finger_map.get(&(Hand::R, Finger::I)).unwrap();
        assert!(j_keys
            .iter()
            .any(|k| k.code.map(|c| c.to_string()) == Some("j".to_string())));
        assert!(j_keys
            .iter()
            .any(|k| k.code.map(|c| c.to_string()) == Some("u".to_string())));

        // Check total number of keys
        let mut total_keys = 0;
        for (_, keys_set) in hand_finger_map.iter() {
            total_keys += keys_set.len();
        }
        assert_eq!(total_keys, ANSI30.len());
    }

    #[test]
    fn test_corpus_processing() {
        let text = "banana";
        let mut corpus = super::corpus::Corpus::new();
        corpus.process_text(text);

        assert_eq!(*corpus.cvc.get("ban").unwrap(), 1);
        assert_eq!(*corpus.cvc.get("nan").unwrap(), 1);
        assert_eq!(corpus.cvc.len(), 2);

        assert_eq!(*corpus.vcv.get("ana").unwrap(), 2);
        assert_eq!(corpus.vcv.len(), 1);

        assert_eq!(*corpus.abab.get("anan").unwrap(), 1);
        assert_eq!(*corpus.abab.get("nana").unwrap(), 1);
        assert_eq!(corpus.abab.len(), 2);

        assert!(corpus.abba.get("bana").is_none());
    }

    #[test]
    fn test_nstroke_iterator() {
        let keymap = Src::Ansi30.keymap();
        let keyboard = super::keyboard::Keyboard::new(&keymap);
        let keys: Vec<_> = keymap.values().cloned().collect();
        let n = 2;
        let mut nstrokes = keyboard.nstrokes(n);

        assert_eq!(nstrokes.clone().count(), keys.len().pow(n as u32));

        // Check first element
        let first = nstrokes.next().unwrap();
        assert_eq!(first, vec![keys[0].clone(), keys[0].clone()]);

        // Check last element
        let last = nstrokes.last().unwrap();
        assert_eq!(
            last,
            vec![keys[keys.len() - 1].clone(), keys[keys.len() - 1].clone()]
        );
    }

    #[test]
    fn test_assign_tier() {
        use super::key::Key;
        use super::tier::assign_tier;
        use core::percentage::T as Percentage;
        use std::collections::HashMap;

        let keymap = crate::key::Src::Ansi30.keymap();
        let char_to_key: HashMap<char, Key> = keymap
            .values()
            .map(|k| {
                (
                    k.code.unwrap().to_string().chars().next().unwrap(),
                    k.clone(),
                )
            })
            .collect();

        let get_stroke =
            |s: &str| -> Vec<Key> { s.chars().map(|c| char_to_key[&c].clone()).collect() };

        // S-tier: same-row, diff fingers, has index
        assert_eq!(
            assign_tier(&get_stroke("asdf")),
            Some(Percentage::new(1.0).unwrap())
        );

        // A-tier: good-row-change-strong-fingers
        assert_eq!(
            assign_tier(&get_stroke("se")),
            Some(Percentage::new(0.8).unwrap())
        );

        // C-tier: wide-good-row-change-weak-fingers or weak-scissors-strong-fingers
        assert_eq!(
            assign_tier(&get_stroke("ed")),
            Some(Percentage::new(0.4).unwrap())
        );

        // SameFinger
        assert_eq!(
            assign_tier(&get_stroke("qq")),
            Some(Percentage::new(0.2).unwrap())
        );

        // None for single key
        assert_eq!(assign_tier(&get_stroke("a")), None);
    }

    #[test]
    fn test_assign_tier_coverage() {
        use super::keyboard::Keyboard;
        use super::tier::assign_tier;

        let keymap = super::key::Src::Ansi30.keymap();
        let keyboard = Keyboard::new(&keymap);

        // Test all 2-strokes
        for stroke in keyboard.nstrokes(2) {
            // If assign_tier panics, the test will fail
            assign_tier(&stroke);
        }

        // Test all 3-strokes
        for stroke in keyboard.nstrokes(3) {
            // If assign_tier panics, the test will fail
            assign_tier(&stroke);
        }
    }
}
