//! Shared http client builder
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder,
};

/// Create a pre-configured ClientBuilder
///
/// This function returns a reqwest::ClientBuilder that has been pre-configured with default headers.
/// Specifically, it sets the `User-Agent` header so that requests from the test suite can be more easily filtered and inspected in the request logs.
pub fn custom_http_client() -> ClientBuilder {
    reqwest::ClientBuilder::new().default_headers(HeaderMap::from_iter([(
        header::USER_AGENT,
        USER_AGENT_HEADER,
    )]))
}

/// User-Agent used for all tests
const USER_AGENT_HEADER: HeaderValue = HeaderValue::from_static("rust-lang/infra-smoke-tests");

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn default_headers_are_addative() {
        let client = custom_http_client()
            .default_headers(HeaderMap::from_iter([(
                header::VIA,
                HeaderValue::from_static("test"),
            )]))
            .build()
            .unwrap();

        let mut server = mockito::Server::new_async().await;

        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .match_header(header::VIA, "test")
            .match_header(
                header::USER_AGENT,
                String::from_utf8_lossy(USER_AGENT_HEADER.as_ref()).as_ref(),
            )
            .create();

        assert!(client.get(server.url()).send().await.is_ok());

        mock.assert();
    }
}
