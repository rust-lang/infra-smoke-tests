//! Test that CloudFront serves the database dump

use std::sync::Arc;

use async_trait::async_trait;

use crate::crates::db_dump::ARTIFACTS;
use crate::http_client::custom_http_client;
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "CloudFront";

/// Test that CloudFront serves the database dump
///
/// The database dump cannot be served directly from Fastly, so it is served from CloudFront.
pub struct CloudFront {
    /// Configuration for this test
    config: Arc<Config>,
}

impl CloudFront {
    /// Create a new instance of the test
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Request the given path and expect a successful response
    async fn request_path_and_expect_success(&self, path: &str) -> TestResult {
        let response = match custom_http_client()
            .build()
            .expect("failed to build reqwest client")
            .head(format!("{}/{}", self.config.cloudfront_url(), path))
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

#[async_trait]
impl Test for CloudFront {
    async fn run(&self) -> TestResult {
        let mut results = Vec::with_capacity(ARTIFACTS.len());

        for artifact in ARTIFACTS {
            let result = self.request_path_and_expect_success(artifact).await;
            results.push(result);
        }

        if let Some(failed) = results.into_iter().find(|result| !result.success()) {
            return failed;
        }

        TestResult::builder().name(NAME).success(true).build()
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

        let mock_tar = server
            .mock("HEAD", "/db-dump.tar.gz")
            .with_status(200)
            .create();

        let mock_zip = server
            .mock("HEAD", "/db-dump.zip")
            .with_status(200)
            .create();

        let result = CloudFront::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock_tar.assert();
        mock_zip.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_with_other_http_responses() {
        let (mut server, config) = setup().await;

        let mock_tar = server
            .mock("HEAD", "/db-dump.tar.gz")
            .with_status(500)
            .create();

        let mock_zip = server
            .mock("HEAD", "/db-dump.zip")
            .with_status(500)
            .create();

        let result = CloudFront::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock_tar.assert();
        mock_zip.assert();

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
