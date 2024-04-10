//! Configuration to test rust-lang/crates.io#6164

use getset::Getters;
#[cfg(test)]
use typed_builder::TypedBuilder;

use crate::environment::Environment;

/// Configuration to test rust-lang/crates.io#6164
///
/// The smoke tests try to download a crate from the different CDNs and check if the CORS headers
/// are set correctly. This requires knowing the respective base URLs, the crate, and its version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
#[cfg_attr(test, derive(TypedBuilder))]
pub struct Config {
    /// The name of the crate
    #[getset(get = "pub")]
    krate: String,

    /// The version with the `+` character
    #[getset(get = "pub")]
    version: String,

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
                krate: "crossbeam".into(),
                version: "0.2.10".into(),
                cloudfront_url: "https://cloudfront-static.staging.crates.io".into(),
                fastly_url: "https://fastly-static.staging.crates.io".into(),
            },
            Environment::Production => Self {
                krate: "axum".into(),
                version: "0.6.10".into(),
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
