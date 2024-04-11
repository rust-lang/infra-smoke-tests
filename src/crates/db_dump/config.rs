//! Configuration to test the database dump

use getset::Getters;
#[cfg(test)]
use typed_builder::TypedBuilder;

use crate::environment::Environment;

/// Configuration to test the database dump
///
/// The smoke tests request the database dump from Fastly and CloudFront and check for the correct
/// response.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
#[cfg_attr(test, derive(TypedBuilder))]
pub struct Config {
    /// The URL for the CloudFront CDN
    #[getset(get = "pub")]
    cloudfront_url: String,

    /// The URL for the Fastly CDN
    #[getset(get = "pub")]
    fastly_url: String,
}

impl Config {
    /// Return the configuration for the given environment
    pub fn for_env(env: Environment) -> Self {
        match env {
            Environment::Staging => Self {
                cloudfront_url: "https://cloudfront-static.staging.crates.io".into(),
                fastly_url: "https://fastly-static.staging.crates.io".into(),
            },
            Environment::Production => Self {
                cloudfront_url: "https://cloudfront-static.crates.io".into(),
                fastly_url: "https://fastly-static.crates.io".into(),
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
