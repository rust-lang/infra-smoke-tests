//! Test Fastly with an encoded URL

use std::sync::Arc;

use async_trait::async_trait;
use reqwest::StatusCode;

use crate::crates::utils::crate_url;
use crate::test::{Test, TestResult};

use super::config::Config;
use super::request_url_and_expect_status;

/// The name of the test
const NAME: &str = "Fastly encoded";

/// Test Fastly with an encoded URL
///
/// This test request a URL with an encoded `+` character from Fastly. The test expects the CDN to
/// return an HTTP 200 OK response.
pub struct FastlyEncoded {
    /// Configuration for this test
    config: Arc<Config>,
}

impl FastlyEncoded {
    /// Create a new instance of the test
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Test for FastlyEncoded {
    async fn run(&self) -> TestResult {
        let url = crate_url(
            self.config.fastly_url(),
            self.config.krate(),
            self.config.version(),
        )
        .replace('+', "%2B");

        request_url_and_expect_status(NAME, &url, StatusCode::OK).await
    }
}

#[cfg(test)]
mod tests {
    use crate::crates::crates_4891::tests::setup;
    use crate::test_utils::*;

    use super::*;

    const KRATE: &str = "rust-cratesio-4891";
    const VERSION: &str = "0.1.0%2B1";

    #[tokio::test]
    async fn succeeds_with_http_200_response() {
        let (mut server, config) = setup(KRATE, VERSION).await;

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{VERSION}.crate").as_str(),
            )
            .with_status(200)
            .create();

        let result = FastlyEncoded::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_with_other_http_responses() {
        let (mut server, config) = setup(KRATE, VERSION).await;

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{VERSION}.crate").as_str(),
            )
            .with_status(403)
            .create();

        let result = FastlyEncoded::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<FastlyEncoded>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<FastlyEncoded>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<FastlyEncoded>();
    }
}
