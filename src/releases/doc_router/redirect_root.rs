//! Redirect `/` or `index.html` to `/stable/`
//!
//! The doc router redirects the paths `/` and `index.html` to `/stable/`.

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::redirect::Policy;

use crate::assertion::{is_redirect, redirects_to};
use crate::http_client::custom_http_client;
use crate::releases::doc_router::Config;
use crate::test::{Test, TestResult};

/// The name of the test
const NAME: &str = "Redirect root path";

/// Redirect `/` or `index.html` to `/stable/`
///
/// The doc router redirects the paths `/` and `index.html` to `/stable/`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RedirectRoot {
    /// Configuration for the test group
    config: Arc<Config>,
}

impl RedirectRoot {
    /// Create a new instance of the test
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl Display for RedirectRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NAME}")
    }
}

#[async_trait]
impl Test for RedirectRoot {
    async fn run(&self) -> TestResult {
        let test_result = TestResult::builder().name(NAME).success(false);

        let response = match custom_http_client()
            // Don't follow the redirect, we want to check the redirect location
            .redirect(Policy::none())
            .build()
            .expect("failed to build reqwest client")
            .get(format!("{}/", self.config.cloudfront_url()))
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                return test_result.message(Some(error.to_string())).build();
            }
        };

        let expected_location = "/stable/";

        if !(is_redirect(&response) && redirects_to(&response, expected_location)) {
            let location = response
                .headers()
                .get("Location")
                .and_then(|header| header.to_str().ok())
                .unwrap_or("<empty location header>");

            return test_result
                .message(Some(format!(
                    "Expected a redirect to {expected_location}, got {location}"
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
            .mock("GET", "/")
            .with_status(302)
            .with_header("Location", "/stable/")
            .create();

        let result = RedirectRoot::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        std::assert_eq!(&None, result.message());
        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_without_redirect() {
        let mut server = mockito::Server::new_async().await;
        let config = Config::builder().cloudfront_url(server.url()).build();

        let mock = server.mock("GET", "/").with_status(200).create();

        let result = RedirectRoot::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<RedirectRoot>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<RedirectRoot>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<RedirectRoot>();
    }
}
