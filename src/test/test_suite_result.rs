//! The result of a test suite

use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use indent::indent_all_by;
use typed_builder::TypedBuilder;

use crate::test::TestGroupResult;

/// The result of a test suite
///
/// A test suite is a collection of test groups. Each test group is a collection of tests. The test
/// suite result contains the results of all the test groups.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, CopyGetters, Getters, TypedBuilder,
)]
pub struct TestSuiteResult {
    /// The name of the test suite
    #[getset(get_copy = "pub")]
    name: &'static str,

    /// The results of the individual test groups in the suite
    #[builder(default)]
    #[getset(get = "pub")]
    results: Vec<TestGroupResult>,
}

impl TestSuiteResult {
    /// Check if all the results in the suite are successful
    ///
    /// A test suite is successful if all the tests in its groups are successful. This method
    /// iterates over the individual test results and checks if all of them are successful.
    pub fn success(&self) -> bool {
        self.results.iter().all(|result| result.success())
    }
}

impl Display for TestSuiteResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let emoji = if self.success() { "✅" } else { "❌" };
        let display = format!("{} {}", emoji, self.name());

        writeln!(f, "{}", display)?;

        for result in &self.results {
            let indented_result = indent_all_by(2, result.to_string());
            write!(f, "{}", indented_result)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::test::TestResult;
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

        let suite_result = TestSuiteResult::builder()
            .name("suite")
            .results(vec![group_result])
            .build();

        assert!(suite_result.success());
    }

    #[test]
    fn success_with_one_failed_test() {
        let successful_group = TestGroupResult::builder()
            .name("success")
            .results(vec![
                TestResult::builder().name("test 1").success(true).build(),
                TestResult::builder().name("test 2").success(true).build(),
            ])
            .build();

        let failing_group = TestGroupResult::builder()
            .name("failure")
            .results(vec![
                TestResult::builder().name("test 1").success(true).build(),
                TestResult::builder().name("test 2").success(false).build(),
            ])
            .build();

        let suite_result = TestSuiteResult::builder()
            .name("suite")
            .results(vec![successful_group, failing_group])
            .build();

        assert!(!suite_result.success());
    }

    #[test]
    fn trait_display_success_without_message() {
        let group_result = TestGroupResult::builder()
            .name("group")
            .results(vec![
                TestResult::builder().name("test 1").success(true).build(),
                TestResult::builder().name("test 2").success(true).build(),
            ])
            .build();

        let suite_result = TestSuiteResult::builder()
            .name("suite")
            .results(vec![group_result])
            .build();

        let expected = indoc! {r#"
            ✅ suite
              ✅ group
                ✅ test 1
                ✅ test 2
        "#};

        assert_eq!(expected, format!("{}", suite_result));
    }

    #[test]
    fn trait_display_failure_with_message() {
        let successful_group = TestGroupResult::builder()
            .name("success")
            .results(vec![
                TestResult::builder().name("test 1").success(true).build(),
                TestResult::builder().name("test 2").success(true).build(),
            ])
            .build();

        let failing_group = TestGroupResult::builder()
            .name("failure")
            .results(vec![
                TestResult::builder().name("test 1").success(true).build(),
                TestResult::builder()
                    .name("test 2")
                    .success(false)
                    .message(Some("message".into()))
                    .build(),
            ])
            .build();

        let suite_result = TestSuiteResult::builder()
            .name("suite")
            .results(vec![successful_group, failing_group])
            .build();

        let expected = indoc! {r#"
            ❌ suite
              ✅ success
                ✅ test 1
                ✅ test 2
              ❌ failure
                ✅ test 1
                ❌ test 2 message
        "#};

        assert_eq!(expected, format!("{}", suite_result));
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
