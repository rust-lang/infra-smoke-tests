//! Configuration to test the doc-router

use getset::Getters;
#[cfg(test)]
use typed_builder::TypedBuilder;

use crate::environment::Environment;

/// Configuration to test the doc-router
///
/// The tests for the doc-router call various endpoints and make assertions about the responses.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
#[cfg_attr(test, derive(TypedBuilder))]
pub struct Config {
    /// The URL for the CloudFront CDN
    #[getset(get = "pub")]
    cloudfront_url: String,
}

impl Config {
    /// Return the configuration for the given environment
    pub fn for_env(env: Environment) -> Self {
        match env {
            Environment::Staging => Self {
                cloudfront_url: "https://dev-doc.rust-lang.org".into(),
            },
            Environment::Production => Self {
                cloudfront_url: "https://doc.rust-lang.org".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_send() {
        assert_send::<Config>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Config>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Config>();
    }
}
