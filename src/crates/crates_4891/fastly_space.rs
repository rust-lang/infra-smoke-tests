//! Test Fastly with a URL including a space character

use async_trait::async_trait;
use reqwest::StatusCode;

use crate::test::{Test, TestResult};

use super::config::Config;
use super::request_url_and_expect_status;

/// The name of the test
const NAME: &str = "Fastly with space";

/// Test Fastly with a URL including a space character
///
/// This test request a URL with a space character from Fastly. The test expects the CDN to return
/// an HTTP 403 Forbidden response.
pub struct FastlySpace<'a> {
    /// Configuration for this test
    config: &'a Config,
}

impl<'a> FastlySpace<'a> {
    /// Create a new instance of the test
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl<'a> Test for FastlySpace<'a> {
    async fn run(&self) -> TestResult {
        let url = format!(
            "{}/crates/{}/{}-{}.crate",
            self.config.fastly_url(),
            self.config.krate(),
            self.config.krate(),
            self.config.version()
        )
        .replace('+', " ");

        request_url_and_expect_status(NAME, &url, StatusCode::FORBIDDEN).await
    }
}

#[cfg(test)]
mod tests {
    use crate::crates::crates_4891::tests::setup;
    use crate::test_utils::*;

    use super::*;

    const KRATE: &str = "rust-cratesio-4891";
    const VERSION: &str = "0.1.0 1";

    #[tokio::test]
    async fn succeeds_with_http_403_response() {
        let (mut server, config) = setup(KRATE, VERSION).await;

        let encoded_version = VERSION.replace(' ', "%20");

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{encoded_version}.crate").as_str(),
            )
            .with_status(403)
            .create();

        let result = FastlySpace::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_with_other_http_responses() {
        let (mut server, config) = setup(KRATE, VERSION).await;

        let encoded_version = VERSION.replace(' ', "%20");

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{encoded_version}.crate").as_str(),
            )
            .with_status(200)
            .create();

        let result = FastlySpace::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<FastlySpace>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<FastlySpace>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<FastlySpace>();
    }
}
