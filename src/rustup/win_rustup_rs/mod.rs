//! Serve Rustup for Windows from short URLs
//!
//! This module tests the three artifacts that can be downloaded from `win.rustup.rs`.

use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::environment::Environment;
use crate::http_client::custom_http_client;
use crate::test::{Test, TestGroup, TestGroupResult, TestResult};

pub use self::aarch64::Aarch64;
pub use self::config::Config;
pub use self::i686::I686;
pub use self::x86_64::X86_64;

mod aarch64;
mod config;
mod i686;
mod x86_64;

/// The name of the test group
const NAME: &str = "win.rustup.rs";

/// Serve Rustup for Windows from short URLs
///
/// This test group tests the three artifacts that can be downloaded from `win.rustup.rs`. Each path
/// on the domain represents a specific architecture and serves the installer as an attachment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct WinRustupRs {
    /// Configuration for the test group
    config: Config,
}

impl WinRustupRs {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Config::for_env(env),
        }
    }
}

impl Display for WinRustupRs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAME)
    }
}

#[async_trait]
impl TestGroup for WinRustupRs {
    async fn run(&self) -> TestGroupResult {
        let tests: Vec<Box<dyn Test>> = vec![
            Box::new(Aarch64::new(&self.config)),
            Box::new(I686::new(&self.config)),
            Box::new(X86_64::new(&self.config)),
        ];

        let mut results = Vec::new();
        for test in tests {
            results.push(test.run().await);
        }

        TestGroupResult::builder()
            .name(NAME)
            .results(results)
            .build()
    }
}

/// Request an artifact from `win.rustup.rs` and expect the correct response
///
/// This function requests the given path and expects the response to contain the correct file as an
/// attachment.
async fn request_installer_and_expect_attachment(name: &'static str, url: &str) -> TestResult {
    let test_result = TestResult::builder().name(name).success(false);

    let response = match custom_http_client()
        .build()
        .expect("failed to build reqwest client")
        .head(url)
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return test_result.message(Some(error.to_string())).build();
        }
    };

    if response.status() != 200 {
        return test_result
            .message(Some(format!(
                "Expected HTTP 200, got HTTP {}",
                response.status()
            )))
            .build();
    }

    if !response
        .headers()
        .get("Content-Type")
        .and_then(|header| header.to_str().ok())
        .is_some_and(|header| header == "application/x-msdownload")
    {
        return test_result
            .message(Some(
                "Expected the Content-Type header to be set to 'application/x-msdownload'".into(),
            ))
            .build();
    }

    if !response
        .headers()
        .get("Content-Disposition")
        .and_then(|header| header.to_str().ok())
        .is_some_and(|header| header.contains(r#"attachment; filename="rustup-init.exe""#))
    {
        return test_result
            .message(Some(
                "Expected the Content-Disposition header to indicate an attachment".into(),
            ))
            .build();
    }

    TestResult::builder().name(name).success(true).build()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_display() {
        let rustup_sh = WinRustupRs::new(Environment::Staging);

        assert_eq!("win.rustup.rs", rustup_sh.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<WinRustupRs>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<WinRustupRs>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<WinRustupRs>();
    }
}
