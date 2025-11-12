//! Core functionality for working with the sferrakl corpus.

/// Logical components that make up the corpus artifacts.
pub const CORPUS_COMPONENTS: &[&str; 3] = &["documents", "annotations", "metadata"];

/// Returns a summary describing how the corpus is processed.
pub fn process_corpus() -> &'static str {
    "sferrakl corpus processed"
}

/// Runs a simple workflow over the corpus data and returns a status message.
pub fn run_workflow() -> String {
    format!("{} successfully", process_corpus())
}

/// Provides a human-readable inspection report for consumers of the corpus.
pub fn inspection_report() -> String {
    let components = CORPUS_COMPONENTS.join(", ");
    format!(
        "Inspection summary: {}. Available components: {}.",
        run_workflow(),
        components
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_mentions_components() {
        let report = inspection_report();
        for component in CORPUS_COMPONENTS {
            assert!(report.contains(component));
        }
    }
}
