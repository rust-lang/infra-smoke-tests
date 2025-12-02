//! Test for index.crates.io

use std::sync::Arc;

use async_trait::async_trait;

use crate::http_client::custom_http_client;
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "index.crates.io";

/// Test that index.crates.io is accessible
pub struct IndexCratesIo {
    /// Configuration for this test
    config: Arc<Config>,
}

impl IndexCratesIo {
    /// Create a new instance of the test
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Test for IndexCratesIo {
    async fn run(&self) -> TestResult {
        let response = match custom_http_client()
            .build()
            .expect("failed to build reqwest client")
            .head(self.config.index_url())
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

        let config = Config::builder().index_url(server.url()).build();

        (server, config)
    }

    #[tokio::test]
    async fn succeeds_with_http_200_response() {
        let (mut server, config) = setup().await;

        let mock = server.mock("HEAD", "/").with_status(200).create();

        let result = IndexCratesIo::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_with_other_http_responses() {
        let (mut server, config) = setup().await;

        let mock = server.mock("HEAD", "/").with_status(500).create();

        let result = IndexCratesIo::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<IndexCratesIo>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<IndexCratesIo>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<IndexCratesIo>();
    }
}
