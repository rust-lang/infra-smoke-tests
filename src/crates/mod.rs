//! Smoke tests for crates.io

use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::crates::issue_4891::Issue4891;
use crate::environment::Environment;
use crate::test::{TestGroup, TestSuite, TestSuiteResult};

mod issue_4891;

/// Smoke tests for crates.io
///
/// This test suite implements the smoke tests for crates.io, mostly importantly its Content
/// Delivery Network. The tests ensure that prior bugs in the configuration are not reintroduced,
/// and that CloudFront and Fastly behave the same.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Crates {
    /// The environment to run the tests in
    env: Environment,
}

impl Crates {
    /// Creates a new instance of the test suite
    pub fn new(env: Environment) -> Self {
        Self { env }
    }
}

impl Display for Crates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "crates.io")
    }
}

#[async_trait]
impl TestSuite for Crates {
    async fn run(&self) -> TestSuiteResult {
        let groups = [Issue4891::new(self.env)];

        let mut results = Vec::with_capacity(groups.len());
        for group in &groups {
            results.push(group.run().await);
        }

        TestSuiteResult::builder()
            .name("crates.io")
            .results(results)
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
