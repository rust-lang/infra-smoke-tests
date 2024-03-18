//! Smoke tests for crates.io

use std::fmt::{Display, Formatter};

use crate::test::{TestSuite, TestSuiteResult};

/// Smoke tests for crates.io
///
/// This test suite implements the smoke tests for crates.io, mostly importantly its Content
/// Delivery Network. The tests ensure that prior bugs in the configuration are not reintroduced,
/// and that CloudFront and Fastly behave the same.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Crates {}

impl Display for Crates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "crates.io")
    }
}

impl TestSuite for Crates {
    async fn run(&self) -> TestSuiteResult {
        TestSuiteResult::builder()
            .name("crates.io")
            .results(Vec::new())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_display() {
        let crates = Crates::default();

        assert_eq!("crates.io", crates.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<Crates>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Crates>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Crates>();
    }
}
