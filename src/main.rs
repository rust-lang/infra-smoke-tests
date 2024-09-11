//! Smoke tests for the infrastructure of the Rust project
//!
//! This command-line application can be used to run smoke tests against the cloud infrastructure of
//! the Rust project. The tests confirm that the infrastructure is working as expected and that no
//! regressions have been introduced.

// Make it easier for future generations to maintain this code base by documenting it.
#![warn(clippy::missing_docs_in_private_items)]

use clap::Parser;

use crate::cli::Cli;
use crate::crates::Crates;
use crate::releases::Releases;
use crate::rustup::Rustup;
use crate::test::{TestSuite, TestSuiteResult};

mod assertion;
mod cli;
mod environment;
mod test;

// Test suites
mod crates;
mod releases;
mod rustup;

#[cfg(test)]
mod test_utils;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let tests: Vec<Box<dyn TestSuite>> = vec![
        Box::new(Crates::new(cli.env())),
        Box::new(Releases::new(cli.env())),
        Box::new(Rustup::new(cli.env())),
    ];

    let mut results: Vec<TestSuiteResult> = Vec::with_capacity(tests.len());
    for test in &tests {
        results.push(test.run().await);
    }

    for result in &results {
        println!("{result}");
    }

    if results.iter().any(|result| !result.success()) {
        std::process::exit(1);
    }
}
