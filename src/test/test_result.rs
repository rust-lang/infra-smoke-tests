//! The result of a test

use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use typed_builder::TypedBuilder;

/// The result of a test
///
/// This struct represents the result of a test. It contains the name of the test, whether it was
/// successful, and an optional message.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, CopyGetters, Getters, TypedBuilder,
)]
pub struct TestResult {
    /// The name of the test
    #[getset(get_copy = "pub")]
    name: &'static str,

    /// Whether the test was successful
    #[getset(get_copy = "pub")]
    success: bool,

    /// An optional message
    #[builder(default)]
    #[getset(get = "pub")]
    message: Option<String>,
}

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let emoji = if self.success { "✅" } else { "❌" };
        let mut display = format!("{} {}", emoji, self.name);

        if let Some(message) = &self.message {
            display.push(' ');
            display.push_str(message);
        }

        write!(f, "{display}")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_display_success_without_message() {
        let outcome = TestResult::builder().name("name").success(true).build();

        assert_eq!(format!("{}", outcome), "✅ name");
    }

    #[test]
    fn trait_display_failure_with_message() {
        let outcome = TestResult::builder()
            .name("name")
            .success(false)
            .message(Some("message".into()))
            .build();

        assert_eq!(format!("{}", outcome), "❌ name message");
    }

    #[test]
    fn trait_send() {
        assert_send::<TestResult>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<TestResult>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<TestResult>();
    }
}
