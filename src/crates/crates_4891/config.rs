//! Configuration to test rust-lang/crates.io#4891

use getset::Getters;
#[cfg(test)]
use typed_builder::TypedBuilder;

use crate::environment::Environment;

/// Configuration to test rust-lang/crates.io#4891
///
/// The smoke tests try to access a crate with a `+` character in its version on all the different
/// Content Delivery Networks. The configuration provides a crate in the different environments that
/// can be used for the tests as well as the URLs for the CDNs.
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
                krate: "rust-cratesio-4891".into(),
                version: "0.1.0+1".into(),
                cloudfront_url: "https://cloudfront-static.staging.crates.io".into(),
                fastly_url: "https://fastly-static.staging.crates.io".into(),
            },
            Environment::Production => Self {
                krate: "libgit2-sys".into(),
                version: "0.12.25+1.3.0".into(),
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
