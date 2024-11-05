//! Redirect requests for database dump to CloudFront
//!
//! This module tests that requests for crates.io's database dump are redirected to and served from
//! CloudFront.

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::task::JoinSet;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult};

pub use self::cloudfront::CloudFront;
pub use self::config::Config;
pub use self::fastly::Fastly;

mod cloudfront;
mod config;
mod fastly;

/// The name of the test group
const NAME: &str = "Database dumps";

/// The different database dumps that are available
const ARTIFACTS: [&str; 2] = ["db-dump.tar.gz", "db-dump.zip"];

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
        let config = Arc::new(self.config.clone());
        let tests: Vec<Box<dyn Test>> = vec![
            Box::new(CloudFront::new(config.clone())),
            Box::new(Fastly::new(config.clone())),
        ];

        let mut js = JoinSet::new();
        for test in tests {
            js.spawn(async move { test.run().await });
        }

        let results = js.join_all().await;

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

        assert_eq!("Database dumps", db_dump.to_string());
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
