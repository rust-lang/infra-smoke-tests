//! Smoke tests for rustup

use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use tokio::task::JoinSet;

use crate::environment::Environment;
use crate::rustup::win_rustup_rs::WinRustupRs;
use crate::test::{TestGroup, TestSuite, TestSuiteResult};

mod win_rustup_rs;

/// Smoke tests for rustup
///
/// This test suite implements the smoke tests for rustup. The tests confirm that the domains of
/// rustup redirect to the correct locations and that the cache invalidations in the CDNs work.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Rustup {
    /// The environment to run the tests in
    env: Environment,
}

impl Rustup {
    /// Creates a new instance of the test suite
    pub fn new(env: Environment) -> Self {
        Self { env }
    }
}

impl Display for Rustup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rustup")
    }
}

#[async_trait]
impl TestSuite for Rustup {
    async fn run(&self) -> TestSuiteResult {
        let groups: Vec<Box<dyn TestGroup>> = vec![Box::new(WinRustupRs::new(self.env))];

        let mut js = JoinSet::new();
        for group in groups {
            js.spawn(async move { group.run().await });
        }

        let results = js.join_all().await;

        TestSuiteResult::builder()
            .name("rustup")
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
        let rustup = Rustup::default();

        assert_eq!("rustup", rustup.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<Rustup>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Rustup>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Rustup>();
    }
}
