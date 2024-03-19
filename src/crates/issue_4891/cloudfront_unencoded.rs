//! Test CloudFront with an un-encoded URL

use url::Url;

use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "CloudFront unencoded";

/// Test CloudFront with an un-encoded URL
///
/// This test request a URL with an un-encoded `+` character from Cloudfront. The test expects the
/// CDN to return an HTTP 403 Forbidden response.
pub struct CloudfrontUnencoded<'a> {
    /// Configuration for this test
    config: &'a Config,
}

impl<'a> CloudfrontUnencoded<'a> {
    /// Create a new instance of the test
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl<'a> Test for CloudfrontUnencoded<'a> {
    async fn run(&self) -> TestResult {
        let url = Url::parse(&format!(
            "{}/crates/{}/{}-{}.crate",
            self.config.cloudfront_url(),
            self.config.krate(),
            self.config.krate(),
            self.config.version()
        ))
        .expect("failed to parse URL for crates on CloudFront");

        let response = match reqwest::get(url).await {
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
                    "Expected HTTP 200 OK, got HTTP {}",
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

    const KRATE: &str = "rust-cratesio-4891";
    const VERSION: &str = "0.1.0+1";

    async fn setup() -> (ServerGuard, Config) {
        let server = mockito::Server::new_async().await;

        let config = Config::builder()
            .krate(KRATE.into())
            .version(VERSION.into())
            .cloudfront_url(server.url())
            .fastly_url(String::new())
            .build();

        (server, config)
    }

    #[tokio::test]
    async fn succeeds_with_http_200_response() {
        let (mut server, config) = setup().await;

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{VERSION}.crate").as_str(),
            )
            .with_status(200)
            .create();

        let result = CloudfrontUnencoded::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_with_other_http_responses() {
        let (mut server, config) = setup().await;

        let mock = server
            .mock(
                "GET",
                format!("/crates/{KRATE}/{KRATE}-{VERSION}.crate").as_str(),
            )
            .with_status(403)
            .create();

        let result = CloudfrontUnencoded::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<CloudfrontUnencoded>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<CloudfrontUnencoded>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<CloudfrontUnencoded>();
    }
}
