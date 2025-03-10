use crate::state::settings::AppSettings;
use crate::state::window::Window;
use std::sync::Arc;

pub struct OliverExplorer {
    pub windows: Vec<Window>,
    pub settings: Arc<AppSettings>,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        let default_settings = Arc::new(AppSettings::default());
        let default_window = Window::new(Arc::clone(&default_settings));
        Self {
            windows: vec![default_window],
            settings: default_settings,
        }
    }
}

impl OliverExplorer {
    pub fn new_window(&mut self) {
        let new_window = Window::new(Arc::clone(&self.settings));
        self.windows.push(new_window);
    }
}
