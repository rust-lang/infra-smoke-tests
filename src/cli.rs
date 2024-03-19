//! Command-line interface to run the smoke tests
//!
//! This module implements the command-line interface that can be used to run the smoke tests. See
//! the `Cli` struct that parses the command-line arguments and options.

use clap::Parser;
use getset::CopyGetters;

use crate::environment::Environment;

/// Smoke Tests for Infrastructure
///
/// This command-line application can be used to run smoke tests against our infrastructure. The
/// tests confirm that the infrastructure is working as expected and that no regressions have been
/// introduced.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, CopyGetters, Parser)]
pub struct Cli {
    /// The environment to run the smoke tests against
    #[arg(long, value_enum, default_value_t)]
    #[getset(get_copy = "pub")]
    env: Environment,
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<Cli>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Cli>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Cli>();
    }
}
