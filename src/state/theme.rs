use eframe::egui;

// Create a default egui style
pub fn default_style() -> egui::Style {
    egui::Style {
        visuals: egui::Visuals {
            panel_fill: egui::Color32::from_rgb(30, 30, 30),
            widgets: egui::style::Widgets {
                inactive: default_widget_visuals(),
                active: egui::style::WidgetVisuals {
                    bg_fill: egui::Color32::from_rgb(45, 45, 45),
                    weak_bg_fill: egui::Color32::from_rgb(45, 45, 45),
                    ..default_widget_visuals()
                },
                hovered: egui::style::WidgetVisuals {
                    bg_fill: egui::Color32::from_rgb(70, 70, 70),
                    weak_bg_fill: egui::Color32::from_rgb(70, 70, 70),
                    ..default_widget_visuals()
                },
                noninteractive: egui::style::WidgetVisuals {
                    bg_fill: egui::Color32::from_rgb(45, 45, 45),
                    weak_bg_fill: egui::Color32::from_rgb(45, 45, 45),
                    ..default_widget_visuals()
                },
                open: default_widget_visuals(),
            },
            window_corner_radius: egui::CornerRadius {
                nw: 10,
                ne: 10,
                sw: 0,
                se: 0,
            },
            window_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180)),
            ..Default::default()
        },
        spacing: egui::Spacing {
            item_spacing: egui::Vec2::new(0.0, 0.0),
            window_margin: egui::Margin {
                left: 8,
                right: 8,
                top: 8,
                bottom: 8,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn default_widget_visuals() -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill: egui::Color32::from_rgb(30, 30, 30),
        weak_bg_fill: egui::Color32::from_rgb(30, 30, 30),
        bg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180)),
        fg_stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180)),
        corner_radius: egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: 0,
            se: 0,
        },
        expansion: 0.0,
    }
}

// used for app specific spacing and sizing
pub struct Layout {
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
            toolbar_height: 40.0,
            tab_width: MinMax {
                min: 40.0,
                max: 200.0,
            },
        }
    }
}
