//! Test that CloudFront serves the database dump

use async_trait::async_trait;
use reqwest::Client;

use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "CloudFront";

/// Test that CloudFront serves the database dump
///
/// The database dump cannot be served directly from Fastly, so it is served from CloudFront.
pub struct CloudFront<'a> {
    /// Configuration for this test
    config: &'a Config,
}

impl<'a> CloudFront<'a> {
    /// Create a new instance of the test
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl<'a> Test for CloudFront<'a> {
    async fn run(&self) -> TestResult {
        let response = match Client::builder()
            .build()
            .expect("failed to build reqwest client")
            .head(format!("{}/db-dump.tar.gz", self.config.cloudfront_url()))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                return TestResult::builder()
                    .name(NAME)
                    .success(false)
                    .message(Some(error.to_string()))
                    .build()
            }
        };

        if response.status().is_success() {
            TestResult::builder().name(NAME).success(true).build()
        } else {
            TestResult::builder()
                .name(NAME)
                .success(false)
                .message(Some(format!(
                    "Expected HTTP 200, got HTTP {}",
                    response.status()
                )))
                .build()
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito::ServerGuard;

    use crate::test_utils::*;

    use super::*;

    pub async fn setup() -> (ServerGuard, Config) {
        let server = mockito::Server::new_async().await;

        let config = Config::builder()
            .cloudfront_url(server.url())
            .fastly_url(server.url())
            .build();

        (server, config)
    }

    #[tokio::test]
    async fn succeeds_with_http_200_response() {
        let (mut server, config) = setup().await;

        let mock = server
            .mock("HEAD", "/db-dump.tar.gz")
            .with_status(200)
            .create();

        let result = CloudFront::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_with_other_http_responses() {
        let (mut server, config) = setup().await;

        let mock = server
            .mock("HEAD", "/db-dump.tar.gz")
            .with_status(500)
            .create();

        let result = CloudFront::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<CloudFront>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<CloudFront>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<CloudFront>();
    }
}
