//! Redirect rustup.sh to sh.rustup.rs
//!
//! This module test that the deprecated `/rustup.sh` path is redirected to `sh.rustup.rs`.

use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use reqwest::redirect::Policy;

use crate::assertion::{is_redirect, redirects_to};
use crate::environment::Environment;
use crate::http_client::custom_http_client;
use crate::test::{Test, TestGroup, TestGroupResult, TestResult};

pub use self::cloudfront::CloudFront;
pub use self::config::Config;
pub use self::fastly::Fastly;

mod cloudfront;
mod config;
mod fastly;

/// The name of the test group
const NAME: &str = "rustup.sh";

/// Redirect rustup.sh to sh.rustup.rs
///
/// The deprecated `/rustup.sh` path is redirected to `sh.rustup.rs`. The body of the HTTP response
/// contains further instructions for users who might not be following the redirect.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RustupSh {
    /// Configuration for the test group
    config: Config,
}

impl RustupSh {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Config::for_env(env),
        }
    }
}

impl Display for RustupSh {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAME)
    }
}

#[async_trait]
impl TestGroup for RustupSh {
    async fn run(&self) -> TestGroupResult {
        let tests: Vec<Box<dyn Test>> = vec![
            Box::new(CloudFront::new(&self.config)),
            Box::new(Fastly::new(&self.config)),
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

/// Request `/rustup.sh` and assert the correct response
///
/// The path `/rustup.sh` is deprecated and is being redirected to `sh.rustup.rs`. This function
/// requests the path from the given base URL and asserts that the response is both a redirect and
/// contains instructions for users who don't follow redirects.
async fn request_rustup_and_expect_redirect(name: &'static str, base_url: &str) -> TestResult {
    let test_result = TestResult::builder().name(name).success(false);

    let response = match custom_http_client()
        // Don't follow the redirect, we want to check the redirect location
        .redirect(Policy::none())
        .build()
        .expect("failed to build reqwest client")
        .get(format!("{}/rustup.sh", base_url))
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return test_result.message(Some(error.to_string())).build();
        }
    };

    let expected_location = "https://sh.rustup.rs";

    if !(is_redirect(&response) && redirects_to(&response, expected_location)) {
        return test_result
            .message(Some(format!(
                "Expected a redirect to {}, got {}",
                expected_location,
                response.url().as_str()
            )))
            .build();
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(error) => {
            return test_result.message(Some(error.to_string())).build();
        }
    };

    if !body.contains("https://sh.rustup.rs") {
        return test_result
            .message(Some("Expected body to link to sh.rustup.rs".into()))
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
        let rustup_sh = RustupSh::new(Environment::Staging);

        assert_eq!("rustup.sh", rustup_sh.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<RustupSh>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<RustupSh>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<RustupSh>();
    }
}
