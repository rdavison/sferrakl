fn main() {
    let summary = sferrakl_corpus::process_corpus();
    println!("Inspector report: {}", summary);
}
