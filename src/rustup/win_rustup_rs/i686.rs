//! Test `win.rustup.rs/i686`

use async_trait::async_trait;

use crate::rustup::win_rustup_rs::request_installer_and_expect_attachment;
use crate::test::{Test, TestResult};

use super::config::Config;

/// The name of the test
const NAME: &str = "i686";

/// Test that `win.rustup.rs/i686` serves the Rustup installer
///
/// This test requests the installer from `win.rustup.rs/i686` and expects the response to contain
/// the correct file as an attachment.
pub struct I686<'a> {
    /// Configuration for this test
    config: &'a Config,
}

impl<'a> I686<'a> {
    /// Create a new instance of the test
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl<'a> Test for I686<'a> {
    async fn run(&self) -> TestResult {
        request_installer_and_expect_attachment(
            NAME,
            &format!("{}/i686", self.config.cloudfront_url()),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[tokio::test]
    async fn succeeds_with_http_200_and_attachment() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder().cloudfront_url(server.url()).build();

        let mock = server
            .mock("HEAD", "/i686")
            .with_status(200)
            .with_header("Content-Type", "application/x-msdownload")
            .with_header(
                "Content-Disposition",
                r#"attachment; filename="rustup-init.exe""#,
            )
            .create();

        let result = I686::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        assert_eq!(&None, result.message());
        assert!(result.success());
    }

    #[tokio::test]
    async fn fails_without_content_type() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder().cloudfront_url(server.url()).build();

        let mock = server
            .mock("HEAD", "/i686")
            .with_status(200)
            .with_header(
                "Content-Disposition",
                r#"attachment; filename="rustup-init.exe""#,
            )
            .create();

        let result = I686::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        let message = result.message().as_ref().unwrap();

        assert!(message.contains("Content-Type"));
        assert!(!result.success());
    }

    #[tokio::test]
    async fn fails_without_content_disposition() {
        let mut server = mockito::Server::new_async().await;

        let config = Config::builder().cloudfront_url(server.url()).build();

        let mock = server
            .mock("HEAD", "/i686")
            .with_status(200)
            .with_header("Content-Type", "application/x-msdownload")
            .create();

        let result = I686::new(&config).run().await;

        // Assert that the mock was called
        mock.assert();

        let message = result.message().as_ref().unwrap();

        assert!(message.contains("Content-Disposition"));
        assert!(!result.success());
    }

    #[test]
    fn trait_send() {
        assert_send::<I686>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<I686>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<I686>();
    }
}
