//! Environments that can be tested by the smoke tests

use clap::ValueEnum;

/// Environments that can be tested by the smoke tests
///
/// This enum represents the environments that can be tested by the smoke tests. Each environment
/// requires its own configuration and has its own set of expectations, and thus requires its own
/// implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, ValueEnum)]
pub enum Environment {
    /// The staging environment
    #[default]
    Staging,

    /// The production environment
    Production,
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_default() {
        assert_eq!(Environment::Staging, Environment::default());
    }

    #[test]
    fn trait_send() {
        assert_send::<Environment>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Environment>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Environment>();
    }
}
