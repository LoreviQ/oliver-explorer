use crate::state::theme::Theme;

pub struct AppSettings {
    pub title: String,
    pub default_url: String,
    pub theme: Theme,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            title: "Oliver Explorer".to_string(),
            default_url: "http://localhost:3333".to_string(),
            theme: Theme::default(),
        }
    }
}
