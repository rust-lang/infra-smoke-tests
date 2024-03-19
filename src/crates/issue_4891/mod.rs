//! Encoded URLs with a + sign fail

use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use reqwest::StatusCode;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult, TestResult};

use self::cloudfront_encoded::CloudfrontEncoded;
use self::cloudfront_space::CloudfrontSpace;
use self::cloudfront_unencoded::CloudfrontUnencoded;
use self::config::Config;

mod cloudfront_encoded;
mod cloudfront_space;
mod cloudfront_unencoded;
mod config;

/// The name of the test group
const NAME: &str = "rust-lang/crates.io#4891";

/// Encoded URLs with a + sign fail
///
/// An issue was reported where requests that encoded the `+` character in the URL would receive an
/// HTTP 403 Forbidden response. The cause for this issue was that the `+` character has a special
/// meaning in S3, which was not considered when uploading crates in the past. The smoke tests
/// ensure that the Content Delivery Networks correctly rewrite the URL to avoid this issue.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Issue4891 {
    /// Configuration for the test group
    config: Config,
}

impl Issue4891 {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Config::for_env(env),
        }
    }
}

impl Display for Issue4891 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAME)
    }
}

#[async_trait]
impl TestGroup for Issue4891 {
    async fn run(&self) -> TestGroupResult {
        let tests: Vec<Box<dyn Test>> = vec![
            Box::new(CloudfrontEncoded::new(&self.config)),
            Box::new(CloudfrontUnencoded::new(&self.config)),
            Box::new(CloudfrontSpace::new(&self.config)),
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

/// Test the given URL and expect the given status code
///
/// This function sends a GET request to the given URL and expects the response to have the given
/// status code. If the request fails, the test will fail with the error message. If the response
/// status code does not match the expected status code, the test will return an unsuccessful
/// `TestResult`.
async fn request_url_and_expect_status(
    name: &'static str,
    url: &str,
    expected_status: StatusCode,
) -> TestResult {
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(error) => {
            return TestResult::builder()
                .name(name)
                .success(false)
                .message(Some(error.to_string()))
                .build()
        }
    };

    if response.status() == expected_status {
        TestResult::builder().name(name).success(true).build()
    } else {
        TestResult::builder()
            .name(name)
            .success(false)
            .message(Some(format!(
                "Expected HTTP {expected_status}, got HTTP {}",
                response.status()
            )))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use mockito::ServerGuard;
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    pub async fn setup(krate: &'static str, version: &'static str) -> (ServerGuard, Config) {
        let server = mockito::Server::new_async().await;

        let config = Config::builder()
            .krate(krate.into())
            .version(version.into())
            .cloudfront_url(server.url())
            .fastly_url(String::new())
            .build();

        (server, config)
    }

    #[test]
    fn trait_display() {
        let issue_4891 = Issue4891::new(Environment::Staging);

        assert_eq!("rust-lang/crates.io#4891", issue_4891.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<Issue4891>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Issue4891>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Issue4891>();
    }
}
