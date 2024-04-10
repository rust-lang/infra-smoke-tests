//! Test CloudFront with an encoded URL

use async_trait::async_trait;
use reqwest::StatusCode;

use crate::crates::utils::crate_url;
use crate::test::{Test, TestResult};

use super::config::Config;
use super::request_url_and_expect_status;

/// The name of the test
const NAME: &str = "CloudFront encoded";

/// Test CloudFront with an encoded URL
///
/// This test request a URL with an encoded `+` character from Cloudfront. The test expects the CDN
/// to return an HTTP 200 OK response.
pub struct CloudfrontEncoded<'a> {
    /// Configuration for this test
    config: &'a Config,
}

impl<'a> CloudfrontEncoded<'a> {
    /// Create a new instance of the test
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl<'a> Test for CloudfrontEncoded<'a> {
    async fn run(&self) -> TestResult {
        let url = crate_url(
            self.config.cloudfront_url(),
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

        let result = CloudfrontEncoded::new(&config).run().await;

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

        let result = CloudfrontEncoded::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<CloudfrontEncoded>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<CloudfrontEncoded>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<CloudfrontEncoded>();
    }
}
