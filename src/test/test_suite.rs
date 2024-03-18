//! A suite of test groups

use crate::test::TestSuiteResult;

/// A suite of test groups
///
/// A test suite is a collection of test groups. Each test group is a collection of tests that are
/// related to each other in some way. The results of the test groups are aggregated to produce the
/// overall result of the test suite.
pub trait TestSuite {
    /// Run the tests in this suite
    async fn run(&self) -> TestSuiteResult;
}
