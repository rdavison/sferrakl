//! Core functionality for working with the sferrakl corpus.

/// Returns a summary describing how the corpus is processed.
pub fn process_corpus() -> &'static str {
    "sferrakl corpus processed"
}

/// Runs a simple workflow over the corpus data and returns a status message.
pub fn run_workflow() -> String {
    format!("{} successfully", process_corpus())
}
