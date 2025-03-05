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
