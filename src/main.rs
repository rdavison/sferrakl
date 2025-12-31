use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
enum SferraklCli {
    BuildCorpus(BuildCorpusArgs),
    Query(QueryArgs),
}

#[derive(Parser, Debug)]
#[command(version, about="Query the corpus db", long_about = None)]
struct QueryArgs {
    selector: String,
    keys: Vec<String>,
    /// corpus db
    #[arg(short, value_name = "FILEPATH")]
    db: PathBuf,
}

#[derive(Parser, Debug)]
#[command(version, about="Corpus file processor", long_about = None)]
struct BuildCorpusArgs {
    /// corpus data
    #[arg(short, value_name = "FILEPATH")]
    input: PathBuf,

    /// database
    #[arg(short, value_name = "FILEPATH")]
    output: PathBuf,
}

fn main() {
    match SferraklCli::parse() {
        SferraklCli::Query(args) => {
            let sferrakl::corpus::Corpus {
                a,
                ab,
                abc,
                a_b,
                aba,
            } = sferrakl::corpus::of_path(args.db).unwrap();
            let hashmap = match args.selector.as_str() {
                "a" => a,
                "ab" => ab,
                "abc" => abc,
                "a_b" => a_b,
                "aba" => aba,
                other => panic!("invalid selector: {}", other),
            };
            for key in args.keys {
                match hashmap.get(&key) {
                    Some(value) => println!("{}: {}", key, value),
                    None => println!("{}: Not Found", key),
                }
            }
        }
        SferraklCli::BuildCorpus(args) => {
            let s = std::fs::read_to_string(args.input).unwrap();
            sferrakl::corpus::of_string(&s).write(args.output).unwrap();
        }
    }
}

#[warn(dead_code)]
fn foobar() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foobar() {
        assert_eq!(foobar(), "Hello, world!");
    }
}
