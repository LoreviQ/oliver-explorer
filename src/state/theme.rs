use eframe::egui::Color32;
use std::sync::RwLock;

// Define a struct to hold all theme values
#[derive(Debug)]
pub struct Theme {
    pub general: ThemeDetails,
    pub accent: ThemeDetails,
    pub frame: FrameDetails,
}

#[derive(Debug)]
pub struct ThemeDetails {
    pub background: Color32,
    pub text: Color32,
    pub hover: Color32,
}

#[derive(Debug)]
pub struct FrameDetails {
    pub text_size: f32,
    pub padding: f32,
    pub toolbar_height: f32,
    pub tab: TabDetails,
}

#[derive(Debug)]
pub struct TabDetails {
    pub width: MinMax,
}

#[derive(Debug)]
pub struct MinMax {
    pub min: f32,
    pub max: f32,
}

// Create a default theme
impl Default for Theme {
    fn default() -> Self {
        Self {
            general: ThemeDetails {
                background: Color32::from_rgb(30, 30, 30),
                text: Color32::from_rgb(230, 230, 230),
                hover: Color32::from_rgb(55, 55, 55),
            },
            accent: ThemeDetails {
                background: Color32::from_rgb(45, 45, 45),
                text: Color32::from_rgb(230, 230, 230),
                hover: Color32::from_rgb(70, 70, 70),
            },
            frame: FrameDetails {
                text_size: 12.0,
                padding: 8.0,
                toolbar_height: 40.0,
                tab: TabDetails {
                    width: MinMax {
                        min: 40.0,
                        max: 200.0,
                    },
                },
            },
        }
    }
}

// Global theme state
static CURRENT_THEME: RwLock<Option<Theme>> = RwLock::new(None);

// Get the current theme (initializes to default if not set)
pub fn get_theme() -> Theme {
    let read_guard = CURRENT_THEME.read().unwrap();

    if let Some(theme) = &*read_guard {
        theme.clone()
    } else {
        drop(read_guard); // Release the read lock before acquiring write lock

        // Initialize with default theme
        let default_theme = Theme::default();
        let mut write_guard = CURRENT_THEME.write().unwrap();
        *write_guard = Some(default_theme.clone());

        default_theme
    }
}

// Clone functions
impl Clone for Theme {
    fn clone(&self) -> Self {
        Self {
            general: self.general.clone(),
            accent: self.accent.clone(),
            frame: self.frame.clone(),
        }
    }
}

impl Clone for ThemeDetails {
    fn clone(&self) -> Self {
        Self {
            background: self.background,
            text: self.text,
            hover: self.hover,
        }
    }
}

impl Clone for FrameDetails {
    fn clone(&self) -> Self {
        Self {
            text_size: self.text_size,
            padding: self.padding,
            toolbar_height: self.toolbar_height,
            tab: self.tab.clone(),
        }
    }
}

impl Clone for TabDetails {
    fn clone(&self) -> Self {
        Self {
            width: self.width.clone(),
        }
    }
}

impl Clone for MinMax {
    fn clone(&self) -> Self {
        Self {
            min: self.min,
            max: self.max,
        }
    }
}
