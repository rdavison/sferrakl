pub mod bigram;
pub mod corpus;
pub mod finger;
pub mod hand;
pub mod hand_finger;
pub mod key;

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
        assert_eq!(bigram_map.0.len(), ANSI30.len() * ANSI30.len());
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
        let text = "abacaba";
        let mut corpus = super::corpus::Corpus::new();
        corpus.process_text(text);

        // ab: distinct-2. "ab", "ba", "ac", "ca"
        assert_eq!(*corpus.ab.get("ab").unwrap(), 2);
        assert_eq!(*corpus.ab.get("ba").unwrap(), 2);
        assert_eq!(*corpus.ab.get("ac").unwrap(), 1);
        assert_eq!(*corpus.ab.get("ca").unwrap(), 1);
        assert_eq!(corpus.ab.len(), 4);

        // aba: repeat-1-distinct-2. "aba", "aca"
        assert_eq!(*corpus.aba.get("aba").unwrap(), 2);
        assert_eq!(*corpus.aba.get("aca").unwrap(), 1);
        assert_eq!(corpus.aba.len(), 2);

        // abc: distinct-3. "bac", "cab"
        assert_eq!(*corpus.abc.get("bac").unwrap(), 1);
        assert_eq!(*corpus.abc.get("cab").unwrap(), 1);
        assert_eq!(corpus.abc.len(), 2);
        
        // a_b: skip-1-distinct-2. "bc", "cb"
        assert_eq!(*corpus.a_b.get("bc").unwrap(), 1);
        assert_eq!(*corpus.a_b.get("cb").unwrap(), 1);
        assert_eq!(corpus.a_b.len(), 2);
    }
}
