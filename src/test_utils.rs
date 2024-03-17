//! Utilities to write unit tests for this crate
//!
//! This module provides utilities to write unit tests for this crate.

/// Assert that a type can be sent across threads
pub fn assert_send<T: Send>() {}

/// Assert that a type can be shared between threads
pub fn assert_sync<T: Sync>() {}

/// Assert that a type can be unpinned
pub fn assert_unpin<T: Unpin>() {}
