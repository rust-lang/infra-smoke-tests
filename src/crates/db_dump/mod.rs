//! Redirect requests for database dump to CloudFront
//!
//! This module tests that requests for crates.io's database dump are redirected to and served from
//! CloudFront.

use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult};

pub use self::cloudfront::CloudFront;
pub use self::config::Config;
pub use self::fastly::Fastly;

mod cloudfront;
mod config;
mod fastly;

/// The name of the test group
const NAME: &str = "db-dump.tar.gz";

/// Redirect requests for database dump to CloudFront
///
/// The database dump has to be served from CloudFront, since the Compute@Edge platform on Fastly
/// does not support files larger than 50MB yet.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DbDump {
    /// Configuration for the test group
    config: Config,
}

impl DbDump {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Config::for_env(env),
        }
    }
}

impl Display for DbDump {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAME)
    }
}

#[async_trait]
impl TestGroup for DbDump {
    async fn run(&self) -> TestGroupResult {
        let tests: Vec<Box<dyn Test>> = vec![
            Box::new(CloudFront::new(&self.config)),
            Box::new(Fastly::new(&self.config)),
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_display() {
        let db_dump = DbDump::new(Environment::Staging);

        assert_eq!("db-dump.tar.gz", db_dump.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<DbDump>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<DbDump>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<DbDump>();
    }
}
