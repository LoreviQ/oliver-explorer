use oliver_explorer::networking;

#[test]
fn test_fetch_url_integration() {
    let result = networking::fetch_url("https://httpbin.org/get");
    assert!(result.is_ok());

    if let Ok(body) = result {
        assert!(body.contains("httpbin"));
    }
}
