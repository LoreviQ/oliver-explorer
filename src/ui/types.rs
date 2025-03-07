use crate::html;
use crate::networking;

pub struct Tab {
    url: String,
    html_content: String,
    pub content: String,
}

impl Tab {
    pub fn new(url: &str) -> Self {
        let html_content = networking::fetch_url(url).unwrap_or_default();
        let content = html::parse_html(&html_content).unwrap_or_default();
        Self {
            url: url.to_string(),
            html_content,
            content,
        }
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }
}
