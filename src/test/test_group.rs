//! A group of tests that belong together

use std::future::Future;

use crate::test::TestGroupResult;

/// A group of tests that belong together
///
/// A test group is a collection of tests that are related to each other. For example, a test group
/// might contain a few tests that together verify a particular feature of the system. The tests are
/// run together and the results are aggregated to produce a single result for the group.
pub trait TestGroup {
    /// Run the tests in this group
    fn run(&self) -> impl Future<Output = TestGroupResult>;
}
