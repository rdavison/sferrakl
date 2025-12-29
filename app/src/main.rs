fn foobar() -> String {
    "Hello, world!".to_string()
}

fn main() {
    let s = foobar();
    println!("{}", s);
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foobar() {
        assert_eq!(foobar(), "Hello, world!");
    }
}
