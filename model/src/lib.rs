pub mod key;

#[cfg(test)]
mod tests {

    use super::key::Src;
    use crate::bigram;
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
}
