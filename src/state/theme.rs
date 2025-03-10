use eframe::egui::Color32;
use std::sync::RwLock;

// Define a struct to hold all theme values
pub struct Theme {
    pub general: ThemeDetails,
    pub active: ThemeDetails,
    pub inactive: ThemeDetails,
    pub frame: FrameDetails,
}

pub struct ThemeDetails {
    pub background: Color32,
    pub text: Color32,
    pub hover: Color32,
}

pub struct FrameDetails {
    pub text_size: f32,
    pub spacing: f32,
    pub padding: f32,
    pub tab: TabDetails,
}

pub struct TabDetails {
    pub width: MinMax,
    pub height: f32,
}

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
            active: ThemeDetails {
                background: Color32::from_rgb(45, 45, 45),
                text: Color32::from_rgb(230, 230, 230),
                hover: Color32::from_rgb(55, 55, 55),
            },
            inactive: ThemeDetails {
                background: Color32::from_rgb(35, 35, 35),
                text: Color32::from_rgb(180, 180, 180),
                hover: Color32::from_rgb(55, 55, 55),
            },
            frame: FrameDetails {
                text_size: 12.0,
                spacing: 4.0,
                padding: 8.0,
                tab: TabDetails {
                    width: MinMax {
                        min: 40.0,
                        max: 200.0,
                    },
                    height: 30.0,
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
            active: self.active.clone(),
            inactive: self.inactive.clone(),
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
            spacing: self.spacing,
            padding: self.padding,
            tab: self.tab.clone(),
        }
    }
}

impl Clone for TabDetails {
    fn clone(&self) -> Self {
        Self {
            width: self.width.clone(),
            height: self.height,
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
