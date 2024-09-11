//! Test that Fastly lists the files in a release

use async_trait::async_trait;

use crate::releases::list_files::request_index_and_expect_loading_files;
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "Fastly";

/// Test that Fastly lists the files in a release
///
/// This module test that requests to the `index.html` in each release folder are rewritten to point
/// to `list-files.html`.
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
        request_index_and_expect_loading_files(
            NAME,
            self.config.fastly_url(),
            self.config.release(),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use indoc::indoc;

    use super::*;

    #[tokio::test]
    async fn succeeds_when_listing_files() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder()
            .cloudfront_url(server.url())
            .fastly_url(server.url())
            .release("2024-09-11".into())
            .build();

        let mock = server
            .mock("GET", "/dist/2024-09-11/index.html")
            .with_status(200)
            .with_body(indoc! {r#"
                Loading directory contents...
            "#})
            .create();

        let result = Fastly::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert_eq!(&None, result.message());
        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_otherwise() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder()
            .cloudfront_url(server.url())
            .fastly_url(server.url())
            .release("2024-09-11".into())
            .build();

        let mock = server
            .mock("GET", "/dist/2024-09-11/index.html")
            .with_status(404)
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
