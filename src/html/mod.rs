use scraper::{Html, Selector};
use std::error::Error;

pub fn parse_html(html: &str) -> Result<String, Box<dyn Error>> {
    let document = Html::parse_document(&html);
    let body_selector = Selector::parse("body").unwrap();
    let text_content = document
        .select(&body_selector)
        .flat_map(|element| element.text())
        .collect::<Vec<_>>()
        .join(" ");
    Ok(text_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_basic() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Test Page</title>
                </head>
                <body>
                    Hello World
                </body>
            </html>
        "#;

        let result = parse_html(html).unwrap();
        assert_eq!(result.trim(), "Hello World");
    }

    #[test]
    fn test_parse_html_with_nested_elements() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Test Page</title>
                </head>
                <body>
                    <div>
                        <p>First paragraph</p>
                        <p>Second <strong>paragraph</strong></p>
                    </div>
                    <div>
                        <span>Additional text</span>
                    </div>
                </body>
            </html>
        "#;

        let result = parse_html(html).unwrap();
        let normalized_result = result.split_whitespace().collect::<Vec<_>>().join(" ");

        assert_eq!(
            normalized_result,
            "First paragraph Second paragraph Additional text"
        );
    }

    #[test]
    fn test_parse_html_empty_body() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Empty Page</title>
                </head>
                <body></body>
            </html>
        "#;

        let result = parse_html(html).unwrap();
        assert_eq!(result.trim(), "");
    }

    #[test]
    fn test_parse_html_no_body() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>No Body Page</title>
                </head>
            </html>
        "#;

        let result = parse_html(html).unwrap();
        assert_eq!(result.trim(), "");
    }

    #[test]
    fn test_parse_html_with_comments() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Test Page</title>
                </head>
                <body>
                    <!-- This is a comment -->
                    <p>Visible text</p>
                    <!-- Another comment -->
                </body>
            </html>
        "#;

        let result = parse_html(html).unwrap();
        assert_eq!(result.trim(), "Visible text");
    }
}
