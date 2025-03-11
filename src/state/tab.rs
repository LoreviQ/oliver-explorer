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
}

impl Tab {
    pub fn new(id: usize, settings: Arc<AppSettings>) -> Self {
        let html_content = networking::fetch_url(&settings.default_url).unwrap_or_default();
        let content = html::parse_html(&html_content).unwrap_or_default();
        Self {
            id,
            url: settings.default_url.to_string(),
            content,
            settings,
            state: TabState::Active,
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == TabState::Active
    }

    // Internal method that only window.rs should call
    pub(in crate::state) fn set_state(&mut self, state: TabState) {
        self.state = state;
    }
}
