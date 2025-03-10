use crate::state::settings::AppSettings;
use crate::state::tab::Tab;
use std::sync::Arc;

pub struct Window {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub settings: Arc<AppSettings>,
}

impl Window {
    pub fn new(settings: Arc<AppSettings>) -> Self {
        let default_tab = Tab::new(Arc::clone(&settings));
        Self {
            tabs: vec![default_tab],
            active_tab: 0,
            settings,
        }
    }

    pub fn new_tab(&mut self) {
        let new_tab = Tab::new(Arc::clone(&self.settings));
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
    }
}
