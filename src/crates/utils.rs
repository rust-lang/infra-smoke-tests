//! Utility functions for working with crates

/// Construct the URL for a crate
///
/// This function constructs the URL for a crate on crates.io. The URL is constructed using the base
/// URL for the CDN, the name of the crate, and the version of the crate.
///
/// # Example
///
/// ```rust
/// use crates::utils::crate_url;
///
/// let base_url = "https://example.com";
/// let krate = "example";
/// let version = "1.0.0";
///
/// let url = crate_url(base_url, krate, version);
/// ```
pub fn crate_url(base_url: &str, krate: &str, version: &str) -> String {
    format!("{}/crates/{}/{}-{}.crate", base_url, krate, krate, version)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn crate_url_constructs_correct_url() {
        let base_url = "https://example.com";
        let krate = "example";
        let version = "1.0.0";

        let expected = "https://example.com/crates/example/example-1.0.0.crate";
        let actual = crate_url(base_url, krate, version);

        assert_eq!(expected, actual);
    }
}
