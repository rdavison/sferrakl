pub mod key;

#[cfg(test)]
mod tests {

    use super::key::{KeyMap, Src};

    #[test]
    fn key_display() {
        let actual = KeyMap::new(Src::Ansi30).to_string();
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
