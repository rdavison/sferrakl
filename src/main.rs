use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
enum SferraklCli {
    BuildCorpus(BuildCorpusArgs),
    Foobar,
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
        SferraklCli::Foobar => println!("Main1 {}", foobar()),
        SferraklCli::BuildCorpus(args) => {
            let s = std::fs::read_to_string(args.input).unwrap();
            sferrakl::corpus::of_string(&s).write(args.output).unwrap();
        }
    }
}

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
