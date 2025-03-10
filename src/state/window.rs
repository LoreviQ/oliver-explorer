use crate::state::settings::AppSettings;
use crate::state::tab::{Tab, TabState};
use std::sync::Arc;

pub struct Window {
    pub tabs: Vec<Tab>,
    pub settings: Arc<AppSettings>,
}

impl Window {
    pub fn new(settings: Arc<AppSettings>) -> Self {
        let default_tab = Tab::new(0, Arc::clone(&settings));
        Self {
            tabs: vec![default_tab],
            settings,
        }
    }

    pub fn new_tab(&mut self) {
        for tab in &mut self.tabs {
            tab.set_state(TabState::Inactive);
        }
        let new_tab = Tab::new(self.tabs.len(), Arc::clone(&self.settings));
        self.tabs.push(new_tab);
    }

    pub fn set_active_tab(&mut self, id: usize) {
        for tab in &mut self.tabs {
            match tab.id == id {
                true => tab.set_state(TabState::Active),
                false => tab.set_state(TabState::Inactive),
            }
        }
    }

    pub fn get_active_tab(&self) -> Result<&Tab, String> {
        match self.tabs.iter().find(|tab| tab.is_active()) {
            Some(tab) => Ok(tab),
            None => Err("No active tab found".to_string()),
        }
    }

    pub fn close_tab(&mut self, id: usize) -> Result<(), String> {
        let Some((tab_to_close, index)) = self
            .tabs
            .iter()
            .enumerate()
            .find(|(_, tab)| tab.id == id)
            .map(|(index, tab)| (tab, index))
        else {
            return Err("Tab not found".to_string());
        };

        // If the tab is not active, remove it
        if !tab_to_close.is_active() {
            self.tabs.remove(index);
            return Ok(());
        }

        match self.tabs.len() {
            1 => {
                // If this is the last remaining tab, request application to close
                std::process::exit(0);
                // TODO: Change to closing window not entire application
            }
            tab_count => {
                if index == tab_count - 1 {
                    self.set_active_tab(self.tabs[index - 1].id);
                } else {
                    self.set_active_tab(self.tabs[index + 1].id);
                }
                self.tabs.remove(index);
                return Ok(());
            }
        }
    }
}
