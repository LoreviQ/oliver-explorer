use crate::state::theme::Layout;
use std::fmt;
use url::Url;

pub struct AppSettings {
    pub title: String,
    pub default_url: Url,
    pub layout: Layout,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            title: "Oliver Explorer".to_string(),
            default_url: Url::parse("http://localhost:3333").unwrap(),
            layout: Layout::default(),
        }
    }
}

impl fmt::Debug for AppSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppSettings")
            .field("title", &self.title)
            .field("default_url", &self.default_url)
            // Intentionally skip the theme field
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert_eq!(settings.title, "Oliver Explorer");
        assert_eq!(
            settings.default_url,
            Url::parse("http://localhost:3333").unwrap()
        );
    }
}
