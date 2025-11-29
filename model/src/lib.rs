pub mod bigram;
pub mod finger;
pub mod hand;
pub mod key;

#[cfg(test)]
mod tests {

    use super::key::Src;
    use crate::bigram;
    use crate::finger::Finger;
    use crate::hand::Hand;
    use crate::key::ANSI30;

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
        assert_eq!(format!("{:?}", Finger::P), "P");
        assert_eq!(format!("{:?}", Finger::R), "R");
        assert_eq!(format!("{:?}", Finger::M), "M");
        assert_eq!(format!("{:?}", Finger::I), "I");
        assert_eq!(format!("{:?}", Finger::T), "T");
    }

    #[test]
    fn hand_enum() {
        assert_eq!(format!("{:?}", Hand::L), "L");
        assert_eq!(format!("{:?}", Hand::R), "R");
    }
}
