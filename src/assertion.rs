//! Shared assertions to test HTTP responses

use reqwest::Response;

/// Check if a response is a redirect
pub fn is_redirect(response: &Response) -> bool {
    response.status().is_redirection()
}

/// Check if a response redirects to the given URL
pub fn redirects_to(response: &Response, url: &str) -> bool {
    response
        .headers()
        .get("Location")
        .and_then(|header| header.to_str().ok())
        .is_some_and(|location| location == url)
}
