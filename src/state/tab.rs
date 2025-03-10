use crate::html;
use crate::networking;
use crate::state::settings::AppSettings;
use std::sync::Arc;

pub struct Tab {
    pub url: String,
    pub content: String,
    pub settings: Arc<AppSettings>,
}

impl Tab {
    pub fn new(settings: Arc<AppSettings>) -> Self {
        let html_content = networking::fetch_url(&settings.default_url).unwrap_or_default();
        let content = html::parse_html(&html_content).unwrap_or_default();
        Self {
            url: settings.default_url.to_string(),
            content,
            settings,
        }
    }
}
