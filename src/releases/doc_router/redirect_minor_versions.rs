//! Redirect requests with only a minor versions
//!
//! Users might access the documentation using a minor version in the URL, for example
//! `/1.65/std/boxed/struct.Box.html`. To avoid returning a HTTP 404 Not Found error for such
//! requests, the doc-router appends `.0` to the minor version and then redirects the request to
//! that version.

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::redirect::Policy;

use crate::assertion::{is_redirect, redirects_to};
use crate::http_client::custom_http_client;
use crate::releases::doc_router::Config;
use crate::test::{Test, TestResult};

/// The name of the test
const NAME: &str = "Redirect minor versions";

/// Redirect requests with only a minor versions
///
/// Users might access the documentation using a minor version in the URL, for example `1.65`. To
/// avoid returning a HTTP 404 Not Found error for such requests, the doc-router appends `.0` to the
/// minor version and then redirects the request to that version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RedirectMinorVersions {
    /// Configuration for the test group
    config: Arc<Config>,
}

impl RedirectMinorVersions {
    /// Create a new instance of the test
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl Display for RedirectMinorVersions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAME)
    }
}

#[async_trait]
impl Test for RedirectMinorVersions {
    async fn run(&self) -> TestResult {
        let test_result = TestResult::builder().name(NAME).success(false);

        let response = match custom_http_client()
            // Don't follow the redirect, we want to check the redirect location
            .redirect(Policy::none())
            .build()
            .expect("failed to build reqwest client")
            .get(format!(
                "{}/1.65/std/boxed/struct.Box.html",
                self.config.cloudfront_url()
            ))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                return test_result.message(Some(error.to_string())).build();
            }
        };

        let expected_location = "/1.65.0/std/boxed/struct.Box.html";

        if !(is_redirect(&response) && redirects_to(&response, expected_location)) {
            let location = response
                .headers()
                .get("Location")
                .and_then(|header| header.to_str().ok())
                .unwrap_or("<empty location header>");

            return test_result
                .message(Some(format!(
                    "Expected a redirect to {}, got {}",
                    expected_location, location
                )))
                .build();
        }

        TestResult::builder().name(NAME).success(true).build()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[tokio::test]
    async fn succeeds_with_redirect_and_body() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder().cloudfront_url(server.url()).build();

        let mock = server
            .mock("GET", "/1.65/std/boxed/struct.Box.html")
            .with_status(302)
            .with_header("Location", "/1.65.0/std/boxed/struct.Box.html")
            .create();

        let result = RedirectMinorVersions::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        std::assert_eq!(&None, result.message());
        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_without_redirect() {
        let mut server = mockito::Server::new_async().await;
        let config = Config::builder().cloudfront_url(server.url()).build();

        let mock = server
            .mock("GET", "/1.65/std/boxed/struct.Box.html")
            .with_status(200)
            .create();

        let result = RedirectMinorVersions::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<RedirectMinorVersions>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<RedirectMinorVersions>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<RedirectMinorVersions>();
    }
}
