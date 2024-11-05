//! Rewrite index.html to list-files.html
//!
//! This module test that requests to the index.html in each release folder are rewritten to point
//! to list-files.html.

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::task::JoinSet;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult, TestResult};

pub use self::cloudfront::CloudFront;
pub use self::config::Config;
pub use self::fastly::Fastly;

mod cloudfront;
mod config;
mod fastly;

/// The name of the test group
const NAME: &str = "list-files.html";

/// Rewrite index.html to list-files.html
///
/// This module test that requests to the index.html in each release folder are rewritten to point
/// to list-files.html.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListFiles {
    /// Configuration for the test group
    config: Config,
}

impl ListFiles {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Config::for_env(env),
        }
    }
}

impl Display for ListFiles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAME)
    }
}

#[async_trait]
impl TestGroup for ListFiles {
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

/// Request a releases `index.html` and assert that it starts loading the files of the release
///
/// The CDN rewrites requests to `index.html` in a release (e.g. `/dist/2024-09-11/index.html`) to
/// point to the `list-files.html` at the root of the bucket, which in turn uses Javascript to fetch
/// a list of all files in the release. The test asserts that the CDN is correctly rewriting the
/// path and returning the script.
async fn request_index_and_expect_loading_files(
    name: &'static str,
    base_url: &str,
    release: &str,
) -> TestResult {
    let test_result = TestResult::builder().name(name).success(false);

    let response = match reqwest::get(format!("{}/dist/{}/index.html", base_url, release)).await {
        Ok(response) => response,
        Err(error) => {
            return test_result.message(Some(error.to_string())).build();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(error) => {
            return test_result.message(Some(error.to_string())).build();
        }
    };

    if !body.contains("Loading directory contents...") {
        return test_result
            .message(Some("Expected body to load directory contents".into()))
            .build();
    }

    TestResult::builder().name(name).success(true).build()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_display() {
        let list_files = ListFiles::new(Environment::Staging);

        assert_eq!("list-files.html", list_files.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<ListFiles>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<ListFiles>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<ListFiles>();
    }
}
