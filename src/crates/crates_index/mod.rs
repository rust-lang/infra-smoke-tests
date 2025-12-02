//! Index domain tests

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::task::JoinSet;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult};

pub use self::config::Config;
pub use self::index_crates_io::IndexCratesIo;

mod config;
mod index_crates_io;

/// The name of the test group
const GROUP_NAME: &str = "Index domains";

/// Test index.crates.io is accessible
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CratesIndex {
    /// Configuration for the test group
    config: Arc<Config>,
}

impl CratesIndex {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Arc::new(Config::for_env(env)),
        }
    }
}

impl Display for CratesIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{GROUP_NAME}")
    }
}

#[async_trait]
impl TestGroup for CratesIndex {
    async fn run(&self) -> TestGroupResult {
        let tests: Vec<Box<dyn Test>> = vec![Box::new(IndexCratesIo::new(self.config.clone()))];

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
        let crates_index = CratesIndex::new(Environment::Staging);

        assert_eq!("Index domains", crates_index.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<CratesIndex>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<CratesIndex>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<CratesIndex>();
    }
}
