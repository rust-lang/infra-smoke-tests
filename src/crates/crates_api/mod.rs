//! crates.io API tests

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::task::JoinSet;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult};

pub use self::api_health::ApiHealth;
pub use self::config::Config;

mod api_health;
mod config;

/// The name of the test group
const GROUP_NAME: &str = "crates.io API";

/// Test crates.io API is accessible
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CratesApi {
    /// Configuration for the test group
    config: Arc<Config>,
}

impl CratesApi {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Arc::new(Config::for_env(env)),
        }
    }
}

impl Display for CratesApi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{GROUP_NAME}")
    }
}

#[async_trait]
impl TestGroup for CratesApi {
    async fn run(&self) -> TestGroupResult {
        let tests: Vec<Box<dyn Test>> = vec![Box::new(ApiHealth::new(self.config.clone()))];

        let mut js = JoinSet::new();
        for test in tests {
            js.spawn(async move { test.run().await });
        }

        let results = js.join_all().await;

        TestGroupResult::builder()
            .name(GROUP_NAME)
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
        let crates_api = CratesApi::new(Environment::Staging);

        assert_eq!("crates.io API", crates_api.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<CratesApi>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<CratesApi>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<CratesApi>();
    }
}
