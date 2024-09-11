//! Configuration to test `list-files.html`

use getset::Getters;
#[cfg(test)]
use typed_builder::TypedBuilder;

use crate::environment::Environment;

/// Configuration to test `list-files.html`
///
/// The smoke tests request the `index.html` file in a release folder and expect it to be list the
/// files in the folder.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
#[cfg_attr(test, derive(TypedBuilder))]
pub struct Config {
    /// The URL for the CloudFront CDN
    #[getset(get = "pub")]
    cloudfront_url: String,

    /// The URL for the Fastly CDN
    #[getset(get = "pub")]
    fastly_url: String,

    /// The date of the release to check
    #[getset(get = "pub")]
    release: String,
}

impl Config {
    /// Return the configuration for the given environment
    pub fn for_env(env: Environment) -> Self {
        match env {
            Environment::Staging => Self {
                cloudfront_url: "https://cloudfront-dev-static.rust-lang.org".into(),
                fastly_url: "https://fastly-dev-static.rust-lang.org".into(),
                release: "2024-09-03".into(),
            },
            Environment::Production => Self {
                cloudfront_url: "https://cloudfront-static.rust-lang.org".into(),
                fastly_url: "https://fastly-static.rust-lang.org".into(),
                release: "2024-09-11".into(),
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
