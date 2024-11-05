//! Test that CloudFront redirects `/rustup.sh` to `sh.rustup.rs`

use std::sync::Arc;

use async_trait::async_trait;

use crate::releases::rustup_sh::request_rustup_and_expect_redirect;
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "CloudFront";

/// Test that CloudFront redirects `/rustup.sh` to `sh.rustup.rs`
///
/// The test requests the deprecated path `/rustup.sh` from CloudFront and checks that it is
/// redirected to `sh.rustup.rs`. The body of the response should contain instructions for the users
/// who don't follow redirects.
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
        request_rustup_and_expect_redirect(NAME, self.config.cloudfront_url()).await
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::test_utils::*;

    use super::*;

    #[tokio::test]
    async fn succeeds_with_redirect_and_body() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder()
            .cloudfront_url(server.url())
            .fastly_url(server.url())
            .build();

        let mock = server
            .mock("GET", "/rustup.sh")
            .with_status(307)
            .with_header("Location", "https://sh.rustup.rs")
            .with_body(indoc! {r#"
                #!/bin/bash
                echo "The location of rustup.sh has moved."
                echo "Run the following command to install from the new location:"
                echo "    curl https://sh.rustup.rs -sSf | sh"
            "#})
            .create();

        let result = CloudFront::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert_eq!(&None, result.message());
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
            .mock("GET", "/rustup.sh")
            .with_status(200)
            .with_body(indoc! {r#"
                #!/bin/bash
                echo "The location of rustup.sh has moved."
                echo "Run the following command to install from the new location:"
                echo "    curl https://sh.rustup.rs -sSf | sh"
            "#})
            .create();

        let result = CloudFront::new(Arc::new(config)).run().await;

        // Assert that the mock was called
        mock.assert();

        assert!(!result.success());
    }

    #[tokio::test]
    async fn fails_without_body() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder()
            .cloudfront_url(server.url())
            .fastly_url(server.url())
            .build();

        let mock = server
            .mock("GET", "/rustup.sh")
            .with_status(307)
            .with_header("Location", "https://sh.rustup.rs")
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
