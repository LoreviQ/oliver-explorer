use crate::html;
use crate::networking;
use crate::state::settings::AppSettings;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum TabState {
    Active,
    Inactive,
}

#[derive(Debug)]
pub struct Tab {
    pub id: usize,
    pub url: String,
    pub content: String,
    pub settings: Arc<AppSettings>,
    state: TabState,
    pub search_buffer: String,
}

impl Tab {
    pub fn new(id: usize, settings: Arc<AppSettings>) -> Self {
        let content = Tab::content_from_url(&settings.default_url).unwrap_or_default();
        Self {
            id,
            url: settings.default_url.to_string(),
            content,
            settings,
            state: TabState::Active,
            search_buffer: String::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == TabState::Active
    }

    // Internal method that only window.rs should call
    pub(in crate::state) fn set_state(&mut self, state: TabState) {
        self.state = state;
    }

    pub fn search(&mut self) -> Result<(), String> {
        let url = self.search_buffer.trim();
        if url.is_empty() {
            return Err("Empty URL".to_string());
        }
        let content = Tab::content_from_url(url)?;
        self.content = content;
        Ok(())
    }

    fn content_from_url(url: &str) -> Result<String, String> {
        // Fetch the URL content
        let html_content = match networking::fetch_url(url) {
            Ok(html_content) => html_content,
            Err(e) => return Err(format!("Failed to fetch URL: {}", e)),
        };
        // Parse the HTML content
        let parsed_content = match html::parse_html(&html_content) {
            Ok(parsed_content) => parsed_content,
            Err(e) => return Err(format!("Failed to parse HTML: {}", e)),
        };
        Ok(parsed_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_new_tab_creation() {
        // Create a new tab
        let settings = Arc::new(AppSettings::default());
        let tab = Tab::new(1, Arc::clone(&settings));

        // Verify the tab properties
        assert_eq!(tab.id, 1);
        assert_eq!(tab.url, settings.default_url);
        assert_eq!(tab.state, TabState::Active);
        assert!(Arc::ptr_eq(&tab.settings, &settings));
        assert_eq!(tab.search_buffer, String::new());
    }

    #[test]
    fn test_is_active() {
        // Create a mock settings object
        let mut tab = Tab::new(1, Arc::new(AppSettings::default()));
        assert!(tab.is_active());

        // Change the state to inactive
        tab.set_state(TabState::Inactive);
        assert!(!tab.is_active());

        // Change back to active
        tab.set_state(TabState::Active);
        assert!(tab.is_active());
    }
}
