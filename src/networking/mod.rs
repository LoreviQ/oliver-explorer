use std::error::Error;
use url::Url;

pub fn fetch_url(url: Url) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;

    // Check if the status code indicates success
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let body = response.text()?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    fn setup_mock_server() -> (mockito::ServerGuard, mockito::Mock) {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/helloworld")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body><h1>Hello World</h1></body></html>")
            .create();

        (server, mock)
    }

    #[test]
    fn test_fetch_url_success() {
        // Fetch from mock server with a valid endpoint
        let (server, mock) = setup_mock_server();
        let endpoint = format!("{}/helloworld", server.url());
        let url = match Url::parse(&endpoint) {
            Ok(url) => url,
            Err(e) => panic!("Failed to parse URL: {}", e),
        };
        let result = fetch_url(url);

        // Verify the request was made and successful
        mock.assert();
        assert!(result.is_ok());

        if let Ok(body) = result {
            assert!(body.contains("Hello World"));
        }
    }

    #[test]
    fn test_fetch_url_invalid_url() {
        // Fetch from mock server with an invalid endpoint
        let (server, _) = setup_mock_server();
        let endpoint = format!("{}/shouldfail", server.url());
        let url = match Url::parse(&endpoint) {
            Ok(url) => url,
            Err(e) => panic!("Failed to parse URL: {}", e),
        };
        let result = fetch_url(url);

        // Verify the request was made and failed
        assert!(result.is_err());
    }
}
