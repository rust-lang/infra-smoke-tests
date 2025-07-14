//! The result of a group of tests

use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use typed_builder::TypedBuilder;

use crate::test::TestResult;

/// The result of a group of tests
///
/// A test group result contains the name of the group and the results of the individual tests in
/// the group.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, CopyGetters, Getters, TypedBuilder,
)]
pub struct TestGroupResult {
    /// The name of the test group
    #[getset(get_copy = "pub")]
    name: &'static str,

    /// The results of the individual tests in the group
    #[builder(default)]
    #[getset(get = "pub")]
    results: Vec<TestResult>,
}

impl TestGroupResult {
    /// Check if all the results in the group are successful
    ///
    /// A test group is successful if all the tests in the group are successful. This method
    /// iterates over the individual test results and checks if all of them are successful.
    pub fn success(&self) -> bool {
        self.results.iter().all(|result| result.success())
    }
}

impl Display for TestGroupResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let emoji = if self.success() { "✅" } else { "❌" };
        let display = format!("{} {}", emoji, self.name());

        writeln!(f, "{display}")?;

        let mut sorted_results = self.results.clone();
        sorted_results.sort();

        for result in sorted_results {
            writeln!(f, "  {result}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn success_with_all_successful_tests() {
        let group_result = TestGroupResult::builder()
            .name("group")
            .results(vec![
                TestResult::builder().name("test 1").success(true).build(),
                TestResult::builder().name("test 2").success(true).build(),
            ])
            .build();

        assert!(group_result.success());
    }

    #[test]
    fn success_with_one_failed_test() {
        let group_result = TestGroupResult::builder()
            .name("group")
            .results(vec![
                TestResult::builder().name("test 1").success(true).build(),
                TestResult::builder().name("test 2").success(false).build(),
            ])
            .build();

        assert!(!group_result.success());
    }

    #[test]
    fn trait_display_success_without_message() {
        let test_result = TestResult::builder().name("test").success(true).build();
        let group_result = TestGroupResult::builder()
            .name("group")
            .results(vec![test_result])
            .build();

        let expected = indoc! {r#"
            ✅ group
              ✅ test
        "#};

        assert_eq!(expected, format!("{}", group_result));
    }

    #[test]
    fn trait_display_failure_with_message() {
        let test_result = TestResult::builder()
            .name("test")
            .success(false)
            .message(Some("message".into()))
            .build();
        let group_result = TestGroupResult::builder()
            .name("group")
            .results(vec![test_result])
            .build();

        let expected = indoc! {r#"
            ❌ group
              ❌ test message
        "#};

        assert_eq!(expected, format!("{}", group_result));
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
