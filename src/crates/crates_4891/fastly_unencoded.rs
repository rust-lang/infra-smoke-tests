//! Test Fastly with an un-encoded URL

use async_trait::async_trait;
use reqwest::StatusCode;

use crate::test::{Test, TestResult};

use super::config::Config;
use super::request_url_and_expect_status;

/// The name of the test
const NAME: &str = "Fastly unencoded";

/// Test Fastly with an un-encoded URL
///
/// This test request a URL with an un-encoded `+` character from Fastly. The test expects the CDN
/// to return an HTTP 200 OK response.
pub struct FastlyUnencoded<'a> {
    /// Configuration for this test
    config: &'a Config,
}

impl<'a> FastlyUnencoded<'a> {
    /// Create a new instance of the test
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl<'a> Test for FastlyUnencoded<'a> {
    async fn run(&self) -> TestResult {
        let url = format!(
            "{}/crates/{}/{}-{}.crate",
            self.config.fastly_url(),
            self.config.krate(),
            self.config.krate(),
            self.config.version()
        );

        request_url_and_expect_status(NAME, &url, StatusCode::OK).await
    }
}

#[cfg(test)]
mod tests {
    use crate::crates::crates_4891::tests::setup;
    use crate::test_utils::*;

    use super::*;

    const KRATE: &str = "rust-cratesio-4891";
    const VERSION: &str = "0.1.0+1";

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

        let result = FastlyUnencoded::new(&config).run().await;

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

        let result = FastlyUnencoded::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<FastlyUnencoded>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<FastlyUnencoded>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<FastlyUnencoded>();
    }
}
