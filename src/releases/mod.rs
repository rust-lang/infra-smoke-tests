//! Smoke tests for Rust releases

use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::environment::Environment;
use crate::releases::list_files::ListFiles;
use crate::releases::rustup_sh::RustupSh;
use crate::test::{TestGroup, TestSuite, TestSuiteResult};

mod list_files;
mod rustup_sh;

/// Smoke tests for Rust releases
///
/// This test suite implements the smoke tests for the Rust releases. The tests confirm that the CDN
/// for releases is working as expected and that no regressions have been introduced.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Releases {
    /// The environment to run the tests in
    env: Environment,
}

impl Releases {
    /// Creates a new instance of the test suite
    pub fn new(env: Environment) -> Self {
        Self { env }
    }
}

impl Display for Releases {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rust releases")
    }
}

#[async_trait]
impl TestSuite for Releases {
    async fn run(&self) -> TestSuiteResult {
        let groups: Vec<Box<dyn TestGroup>> = vec![
            Box::new(ListFiles::new(self.env)),
            Box::new(RustupSh::new(self.env)),
        ];

        let mut results = Vec::with_capacity(groups.len());
        for group in &groups {
            results.push(group.run().await);
        }

        TestSuiteResult::builder()
            .name("Rust releases")
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
        let releases = Releases::default();

        assert_eq!("Rust releases", releases.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<Releases>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Releases>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Releases>();
    }
}
