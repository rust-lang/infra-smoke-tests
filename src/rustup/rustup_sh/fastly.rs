//! Test that Fastly redirects `/rustup.sh` to `sh.rustup.rs`

use async_trait::async_trait;

use crate::rustup::rustup_sh::request_rustup_and_expect_redirect;
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "Fastly";

/// Test that Fastly redirects `/rustup.sh` to `sh.rustup.rs`
///
/// The test requests the deprecated path `/rustup.sh` from Fastly and checks that it is redirected
/// to `sh.rustup.rs`. The body of the response should contain instructions for the users who don't
/// follow redirects.
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
        request_rustup_and_expect_redirect(NAME, self.config.fastly_url()).await
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

        let result = Fastly::new(&config).run().await;

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

        let result = Fastly::new(&config).run().await;

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
