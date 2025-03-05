use scraper::{Html, Selector};
use std::error::Error;

fn main() {
    let url = "https://www.oliver.tj/";
    let html = fetch_url(url).unwrap();
    let text_content = parse_html(&html).unwrap();
    println!("Text content: {}", text_content);
}

fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}

fn parse_html(html: &str) -> Result<String, Box<dyn Error>> {
    let document = Html::parse_document(&html);
    let body_selector = Selector::parse("body").unwrap();
    let text_content = document
        .select(&body_selector)
        .flat_map(|element| element.text())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(text_content)
}
