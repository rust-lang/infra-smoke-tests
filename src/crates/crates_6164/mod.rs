//! Missing CORS headers for crate downloads
//!
//! This module implements smoke tests for <https://github.com/rust-lang/crates.io/issues/6164>,
//! which reported an issue with missing CORS headers.

use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::environment::Environment;
use crate::test::{Test, TestGroup, TestGroupResult, TestResult};

pub use self::cloudfront::CloudFront;
pub use self::config::Config;
pub use self::fastly::Fastly;

mod cloudfront;
mod config;
mod fastly;

/// The name of the test group
const NAME: &str = "rust-lang/crates.io#6164 - CORS headers";

/// Missing CORS header for downloads
///
/// The Fastly service for `static.crates.io` did not always set the `Access-Control-Allow-Origin`
/// header, which caused issues for some users. This test group ensures that the header is always
/// set.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Crates6164 {
    /// Configuration for the test group
    config: Config,
}

impl Crates6164 {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Config::for_env(env),
        }
    }
}

impl Display for Crates6164 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAME)
    }
}

#[async_trait]
impl TestGroup for Crates6164 {
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

/// Test the given URL and expect the CORS header to be set
///
/// This function sends a GET request to the given URL and expects the response to have the
/// `Access-Control-Allow-Origin` header set.
async fn request_url_and_expect_cors_header(name: &'static str, url: &str) -> TestResult {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Origin",
        HeaderValue::from_str("https://example.com").expect("failed to parse header value"),
    );

    let response = match Client::builder()
        .default_headers(headers)
        .build()
        .expect("failed to build reqwest client")
        .get(url)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return TestResult::builder()
                .name(name)
                .success(false)
                .message(Some(error.to_string()))
                .build()
        }
    };

    if response
        .headers()
        .get("Access-Control-Allow-Origin")
        .and_then(|header| header.to_str().ok())
        .is_some_and(|header| header == "*")
    {
        TestResult::builder().name(name).success(true).build()
    } else {
        TestResult::builder()
            .name(name)
            .success(false)
            .message(Some(
                "Expected the Access-Control-Allow-Origin header to be set to '*'".into(),
            ))
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
            .fastly_url(server.url())
            .build();

        (server, config)
    }

    #[test]
    fn trait_display() {
        let crates_6164 = Crates6164::new(Environment::Staging);

        assert_eq!(
            "rust-lang/crates.io#6164 - CORS headers",
            crates_6164.to_string()
        );
    }

    #[test]
    fn trait_send() {
        assert_send::<Crates6164>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Crates6164>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Crates6164>();
    }
}
