//! Test the CORS headers on CloudFront

use std::sync::Arc;

use async_trait::async_trait;

use crate::crates::utils::crate_url;
use crate::test::{Test, TestResult};

use super::config::Config;
use super::request_url_and_expect_cors_header;

/// The name of the test
const NAME: &str = "CloudFront";

/// Test the CORS headers on CloudFront
///
/// This test requests a crate from CloudFront and expects the response to have the correct CORS
/// headers.
pub struct CloudFront {
    /// Configuration for this test
    config: Arc<Config>,
}

impl CloudFront {
    /// Create a new instance of the test
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Test for CloudFront {
    async fn run(&self) -> TestResult {
        let url = crate_url(
            self.config.cloudfront_url(),
            self.config.krate(),
            self.config.version(),
        );

        request_url_and_expect_cors_header(NAME, &url).await
    }
}

#[cfg(test)]
mod tests {
    use crate::crates::crates_6164::tests::setup;
    use crate::test_utils::*;

    use super::*;

    const KRATE: &str = "crates-6164";
    const VERSION: &str = "1.0.0";

    #[tokio::test]
    async fn succeeds_with_cors_header() {
        let (mut server, config) = setup(KRATE, VERSION).await;

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{VERSION}.crate").as_str(),
            )
            .with_status(200)
            .with_header("Access-Control-Allow-Origin", "*")
            .create();

        let result = CloudFront::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_without_cors_header() {
        let (mut server, config) = setup(KRATE, VERSION).await;

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{VERSION}.crate").as_str(),
            )
            .with_status(200)
            .create();

        let result = CloudFront::new(Arc::new(config)).run().await;

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
