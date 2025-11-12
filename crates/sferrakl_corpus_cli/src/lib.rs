//! Command-line interface utilities for the sferrakl corpus tools.

use sferrakl_corpus::run_workflow;

/// Entry point invoked by binary crates to execute the CLI workflow.
pub fn run_cli() {
    let status = run_workflow();
    println!("{}", status);
}
