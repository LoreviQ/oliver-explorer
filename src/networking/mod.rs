use std::error::Error;

pub fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_url_success() {
        // Note: This test depends on an external service
        let result = fetch_url("https://httpbin.org/get");
        assert!(result.is_ok());

        if let Ok(body) = result {
            assert!(body.contains("httpbin"));
        }
    }

    #[test]
    fn test_fetch_url_invalid_url() {
        let result = fetch_url("https://this-does-not-exist-abcxyz.org");
        assert!(result.is_err());
    }
}
