//! Configuration for crates.io API tests

use getset::Getters;
#[cfg(test)]
use typed_builder::TypedBuilder;

use crate::environment::Environment;

/// Configuration for crates.io API tests
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
#[cfg_attr(test, derive(TypedBuilder))]
pub struct Config {
    /// The URL for the crates.io API
    #[getset(get = "pub")]
    api_url: String,
}

impl Config {
    /// Return the configuration for the given environment
    pub fn for_env(env: Environment) -> Self {
        // Use the summary endpoint as a simple health check
        const PATH: &str = "/api/v1/summary";
        match env {
            Environment::Staging => Self {
                api_url: format!("https://staging.crates.io{PATH}"),
            },
            Environment::Production => Self {
                api_url: format!("https://crates.io{PATH}"),
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
