//! Test that Fastly redirects to CloudFront

use async_trait::async_trait;
use reqwest::redirect::Policy;
use reqwest::Client;

use crate::assertion::{is_redirect, redirects_to};
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "Fastly";

/// Test that Fastly redirects to CloudFront
///
/// The database dump cannot be served directly from Fastly, so it should redirect to CloudFront.
pub struct Fastly<'a> {
    /// Configuration for this test
    config: &'a Config,
}

impl<'a> Fastly<'a> {
    /// Create a new instance of the test
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl<'a> Test for Fastly<'a> {
    async fn run(&self) -> TestResult {
        let response = match Client::builder()
            // Don't follow the redirect, we want to check the redirect location
            .redirect(Policy::none())
            .build()
            .expect("failed to build reqwest client")
            .head(format!("{}/db-dump.tar.gz", self.config.fastly_url()))
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

        let expected_location = format!("{}/db-dump.tar.gz", self.config.cloudfront_url());

        if is_redirect(&response) && redirects_to(&response, &expected_location) {
            TestResult::builder().name(NAME).success(true).build()
        } else {
            TestResult::builder()
                .name(NAME)
                .success(false)
                .message(Some(format!(
                    "Expected a redirect to {}, got {}",
                    expected_location,
                    response.url().as_str()
                )))
                .build()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[tokio::test]
    async fn succeeds_with_redirect() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder()
            .cloudfront_url("https://cloudfront".into())
            .fastly_url(server.url())
            .build();

        let mock = server
            .mock("HEAD", "/db-dump.tar.gz")
            .with_status(307)
            .with_header("Location", "https://cloudfront/db-dump.tar.gz")
            .create();

        let result = Fastly::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_without_redirect() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder()
            .cloudfront_url(server.url())
            .fastly_url(server.url())
            .build();

        let mock = server
            .mock("HEAD", "/db-dump.tar.gz")
            .with_status(200)
            .create();

        let result = Fastly::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<Fastly>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Fastly>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Fastly>();
    }
}
