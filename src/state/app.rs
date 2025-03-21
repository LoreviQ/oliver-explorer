use crate::state::settings::AppSettings;
use crate::state::window::Window;
use std::sync::Arc;

pub struct OliverExplorer {
    pub windows: Vec<Window>,
    pub settings: Arc<AppSettings>,
    next_window_id: usize,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        let default_settings = Arc::new(AppSettings::default());
        let default_window = Window::new(0, Arc::clone(&default_settings));
        Self {
            windows: vec![default_window],
            settings: default_settings,
            next_window_id: 1,
        }
    }
}

impl OliverExplorer {
    pub fn new_window(&mut self) {
        let new_window = Window::new(self.next_window_id, Arc::clone(&self.settings));
        self.windows.push(new_window);
        self.next_window_id += 1;
    }

    pub fn close_window(&mut self, id: usize) {
        let index = self.windows.iter().position(|w| w.id == id).unwrap();
        self.windows.remove(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_initialization() {
        // Create a default app instance
        let app = OliverExplorer::default();
        assert_eq!(app.windows.len(), 1);
        assert!(Arc::strong_count(&app.settings) >= 2);
    }

    #[test]
    fn test_adding_windows() {
        // Create a default app instance
        let mut app = OliverExplorer::default();
        assert_eq!(app.windows.len(), 1);

        // Add a new window
        app.new_window();
        assert_eq!(app.windows.len(), 2);

        // Add another window
        app.new_window();
        assert_eq!(app.windows.len(), 3);

        // Verify that all windows share the same settings
        let settings_ptr = Arc::as_ptr(&app.settings);
        for window in &app.windows {
            assert_eq!(Arc::as_ptr(&window.settings), settings_ptr);
        }

        // The reference count should be at least 4 (1 for app + 3 for windows)
        assert!(Arc::strong_count(&app.settings) >= 4);
    }
}
