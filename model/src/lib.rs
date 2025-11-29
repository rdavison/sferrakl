pub mod bigram;
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
        let actual = Src::Ansi30.keycodes().to_string();
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
}
