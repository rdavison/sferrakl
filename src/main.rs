use sferrakl::corpus;

fn main() {
    let s = corpus::foobar();
    println!("Main1 {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foobar() {
        assert_eq!(corpus::foobar(), "Hello, world!");
    }
}
