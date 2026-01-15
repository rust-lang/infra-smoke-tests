//! Test for crates.io API health

use std::sync::Arc;

use async_trait::async_trait;

use crate::http_client::custom_http_client;
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "crates.io API";

/// Test that the crates.io API is accessible
pub struct ApiHealth {
    /// Configuration for this test
    config: Arc<Config>,
}

impl ApiHealth {
    /// Create a new instance of the test
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Test for ApiHealth {
    async fn run(&self) -> TestResult {
        let response = match custom_http_client()
            .build()
            .expect("failed to build reqwest client")
            .get(self.config.api_url())
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

        let config = Config::builder().api_url(server.url()).build();

        (server, config)
    }

    #[tokio::test]
    async fn succeeds_with_http_200_response() {
        let (mut server, config) = setup().await;

        let mock = server.mock("GET", "/").with_status(200).create();

        let result = ApiHealth::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_with_other_http_responses() {
        let (mut server, config) = setup().await;

        let mock = server.mock("GET", "/").with_status(500).create();

        let result = ApiHealth::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<ApiHealth>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<ApiHealth>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<ApiHealth>();
    }
}
