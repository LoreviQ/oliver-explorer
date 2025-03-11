use crate::state::theme::Theme;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert_eq!(settings.title, "Oliver Explorer");
        assert_eq!(settings.default_url, "http://localhost:3333");
    }
}
