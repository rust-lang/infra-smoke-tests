//! Serve the documentation for Rust's standard library
//!
//! This module tests the so-called "doc-router" that serves the documentation for Rust's standard
//! library. The documentation is only served through CloudFront.

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult};

pub use self::config::Config;
use self::redirect_minor_versions::RedirectMinorVersions;
use self::redirect_root::RedirectRoot;

mod config;
mod redirect_minor_versions;
mod redirect_root;

/// The name of the test group
const NAME: &str = "doc-router";

/// Serve the documentation for Rust's standard library
///
/// This module tests the so-called "doc-router" that serves the documentation for Rust's standard
/// library.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DocRouter {
    /// Configuration for the test group
    config: Arc<Config>,
}

impl DocRouter {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Arc::new(Config::for_env(env)),
        }
    }
}

impl Display for DocRouter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NAME}")
    }
}

#[async_trait]
impl TestGroup for DocRouter {
    async fn run(&self) -> TestGroupResult {
        let tests: Vec<Box<dyn Test>> = vec![
            Box::new(RedirectMinorVersions::new(self.config.clone())),
            Box::new(RedirectRoot::new(self.config.clone())),
        ];

        let mut results = Vec::new();
        for test in tests {
            results.push(test.run().await);
        }

        TestGroupResult::builder()
            .name(NAME)
            .results(results)
            .build()
    }
}
