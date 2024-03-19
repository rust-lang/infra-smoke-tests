//! Configuration to test rust-lang/crates.io#4891

use getset::Getters;

use crate::environment::Environment;

/// Configuration to test rust-lang/crates.io#4891
///
/// The smoke tests try to access a crate with a `+` character in its version on all the different
/// Content Delivery Networks. The configuration provides a crate in the different environments that
/// can be used for the tests as well as the URLs for the CDNs.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
pub struct Config {
    /// The name of the crate
    #[get(get = "pub")]
    krate: &'static str,

    /// The version with the `+` character
    #[get(get = "pub")]
    version: &'static str,

    /// The URL for the CloudFront CDN
    #[get(get = "pub")]
    cloudfront_url: &'static str,

    /// The URL for the Fastly CDN
    #[get(get = "pub")]
    fastly_url: &'static str,
}

impl Config {
    /// Return the configuration for the given environment
    pub fn for_env(env: Environment) -> Self {
        match env {
            Environment::Staging => Self {
                krate: "rust-cratesio-4891",
                version: "0.1.0+1",
                cloudfront_url: "https://cloudfront-static.staging.crates.io",
                fastly_url: "https://fastly-static.staging.crates.io",
            },
            Environment::Production => Self {
                krate: "libgit2-sys",
                version: "0.12.25+1.3.0",
                cloudfront_url: "https://cloudfront-static.crates.io",
                fastly_url: "https://fastly-static.crates.io",
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
