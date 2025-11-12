//! Binary that surfaces a focused inspection report for the sferrakl corpus.

fn main() {
    let report = sferrakl_corpus::inspection_report();
    println!("{}", report);
}
