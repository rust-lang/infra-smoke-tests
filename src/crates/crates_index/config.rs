//! Configuration for index domain tests

use getset::Getters;
#[cfg(test)]
use typed_builder::TypedBuilder;

use crate::environment::Environment;

/// Configuration for index domain tests
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
#[cfg_attr(test, derive(TypedBuilder))]
pub struct Config {
    /// The URL for the index.crates.io domain
    #[getset(get = "pub")]
    index_url: String,
}

impl Config {
    /// Return the configuration for the given environment
    pub fn for_env(env: Environment) -> Self {
        // Path to request a dummy crate from the index
        const URL: &str = "/3/f/foo";
        match env {
            Environment::Staging => Self {
                index_url: format!("https://index.staging.crates.io{URL}"),
            },
            Environment::Production => Self {
                index_url: format!("https://index.crates.io{URL}"),
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
