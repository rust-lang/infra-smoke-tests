//! Smoke tests for the infrastructure of the Rust project
//!
//! This command-line application can be used to run smoke tests against the cloud infrastructure of
//! the Rust project. The tests confirm that the infrastructure is working as expected and that no
//! regressions have been introduced.

// Make it easier for future generations to maintain this code base by documenting it.
#![warn(clippy::missing_docs_in_private_items)]

mod cli;
mod environment;

#[cfg(test)]
mod test_utils;

fn main() {
    println!("Hello, world!");
}
