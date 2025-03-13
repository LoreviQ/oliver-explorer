use eframe::egui;

pub struct Theme {
    pub style: egui::Style,
    pub frame: Layout,
}

// Create a default theme
impl Default for Theme {
    fn default() -> Self {
        Self {
            style: egui::Style::default(),
            frame: Layout::default(),
        }
    }
}
/* TODO: Fix with new style system
impl Default for Theme {
    fn default() -> Self {
        Self {
            general: ThemeDetails {
                background: egui::Color32::from_rgb(30, 30, 30),
                text: egui::Color32::from_rgb(230, 230, 230),
                hover: egui::Color32::from_rgb(55, 55, 55),
            },
            accent: ThemeDetails {
                background: egui::Color32::from_rgb(45, 45, 45),
                text: egui::Color32::from_rgb(230, 230, 230),
                hover: egui::Color32::from_rgb(70, 70, 70),
            },
            frame: FrameDetails
        }
    }
}
*/

pub struct Layout {
    pub text_size: f32,
    pub padding: f32,
    pub toolbar_height: f32,
    pub tab_width: MinMax,
}

pub struct MinMax {
    pub min: f32,
    pub max: f32,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            text_size: 12.0,
            padding: 8.0,
            toolbar_height: 40.0,
            tab_width: MinMax {
                min: 40.0,
                max: 200.0,
            },
        }
    }
}
