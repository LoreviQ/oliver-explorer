use crate::state::settings::AppSettings;
use crate::state::tab::{Tab, TabState};
use std::sync::Arc;

pub struct Window {
    pub tabs: Vec<Tab>,
    pub settings: Arc<AppSettings>,
    next_tab_id: usize,
}

impl Window {
    pub fn new(settings: Arc<AppSettings>) -> Self {
        let default_tab = Tab::new(0, Arc::clone(&settings));
        Self {
            tabs: vec![default_tab],
            settings,
            next_tab_id: 1,
        }
    }

    pub fn new_tab(&mut self) {
        for tab in &mut self.tabs {
            tab.set_state(TabState::Inactive);
        }
        let new_tab = Tab::new(self.next_tab_id, Arc::clone(&self.settings));
        self.tabs.push(new_tab);
        self.next_tab_id += 1;
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
        // Find the tab to close and its index
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

        // If the tab is active, we need to find the next tab to activate
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let settings = Arc::new(AppSettings::default());
        let window = Window::new(Arc::clone(&settings));

        // Check window has correct settings
        assert!(Arc::ptr_eq(&window.settings, &settings));
        assert_eq!(window.tabs.len(), 1);
        assert!(window.tabs[0].is_active());
        assert_eq!(window.tabs[0].id, 0);
    }

    #[test]
    fn test_new_tab() {
        let settings = Arc::new(AppSettings::default());
        let mut window = Window::new(Arc::clone(&settings));

        // Initially one tab
        assert_eq!(window.tabs.len(), 1);
        assert!(window.tabs[0].is_active());

        // Create a new tab - should be active with id 1
        window.new_tab();
        assert_eq!(window.tabs.len(), 2);
        assert!(!window.tabs[0].is_active());
        assert!(window.tabs[1].is_active());
        assert_eq!(window.tabs[1].id, 1);
    }

    #[test]
    fn test_set_active_tab() {
        let settings = Arc::new(AppSettings::default());
        let mut window = Window::new(Arc::clone(&settings));

        // Add a second tab
        window.new_tab();

        // Second tab (id 1) should be active
        assert!(!window.tabs[0].is_active());
        assert!(window.tabs[1].is_active());

        // Set first tab as active
        window.set_active_tab(0);

        // First tab should now be active, second inactive
        assert!(window.tabs[0].is_active());
        assert!(!window.tabs[1].is_active());
    }

    #[test]
    fn test_get_active_tab() {
        let settings = Arc::new(AppSettings::default());
        let mut window = Window::new(Arc::clone(&settings));

        // Add a second tab (which becomes active)
        window.new_tab();

        // Get active tab should return the second tab
        let active_tab = window.get_active_tab().unwrap();
        assert_eq!(active_tab.id, 1);

        // Set first tab as active
        window.set_active_tab(0);

        // Get active tab should now return the first tab
        let active_tab = window.get_active_tab().unwrap();
        assert_eq!(active_tab.id, 0);
    }

    #[test]
    fn test_close_inactive_tab() {
        let settings = Arc::new(AppSettings::default());
        let mut window = Window::new(Arc::clone(&settings));

        // Add a second tab
        window.new_tab();

        // Close the first tab (which is inactive)
        let result = window.close_tab(0);
        assert!(result.is_ok());

        // Should have one tab left
        assert_eq!(window.tabs.len(), 1);

        // The remaining tab should be active and have id 1
        assert!(window.tabs[0].is_active());
        assert_eq!(window.tabs[0].id, 1);
    }

    #[test]
    fn test_close_active_tab_with_previous() {
        let settings = Arc::new(AppSettings::default());
        let mut window = Window::new(Arc::clone(&settings));

        // Add two more tabs
        window.new_tab(); // id 1
        window.new_tab(); // id 2

        // Close the last tab (which is active)
        let result = window.close_tab(2);
        assert!(result.is_ok());

        // Should have two tabs left
        assert_eq!(window.tabs.len(), 2);

        // Tab with id 1 should now be active
        let active_tab = window.get_active_tab().unwrap();
        assert_eq!(active_tab.id, 1);
    }

    #[test]
    fn test_close_active_tab_with_next() {
        let settings = Arc::new(AppSettings::default());
        let mut window = Window::new(Arc::clone(&settings));

        // Add a second tab
        window.new_tab(); // id 1

        // Set first tab as active
        window.set_active_tab(0);

        // Close the first tab (which is active)
        let result = window.close_tab(0);
        assert!(result.is_ok());

        // Should have one tab left
        assert_eq!(window.tabs.len(), 1);

        // The remaining tab should be active and have id 1
        assert!(window.tabs[0].is_active());
        assert_eq!(window.tabs[0].id, 1);
    }

    #[test]
    fn test_close_nonexistent_tab() {
        let settings = Arc::new(AppSettings::default());
        let mut window = Window::new(Arc::clone(&settings));

        // Try to close a tab that doesn't exist
        let result = window.close_tab(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Tab not found");

        // Should still have one tab
        assert_eq!(window.tabs.len(), 1);
    }

    // Note: We can't easily test the case where the last tab is closed
    // because it calls std::process::exit(0)
}
