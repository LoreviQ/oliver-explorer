use eframe::egui;

pub struct Theme {
    pub style: egui::Style,
    pub frame: Layout,
}

// Create a default theme
impl Default for Theme {
    fn default() -> Self {
        Self {
            style: egui::Style {
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
                            bg_fill: egui::Color32::from_rgb(55, 55, 55),
                            weak_bg_fill: egui::Color32::from_rgb(55, 55, 55),
                            ..default_widget_visuals()
                        },
                        noninteractive: egui::style::WidgetVisuals {
                            bg_fill: egui::Color32::from_rgb(45, 45, 45),
                            weak_bg_fill: egui::Color32::from_rgb(45, 45, 45),
                            ..default_widget_visuals()
                        },
                        open: default_widget_visuals(),
                    },
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
            },
            frame: Layout::default(),
        }
    }
}

fn default_widget_visuals() -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill: egui::Color32::from_rgb(30, 30, 30),
        weak_bg_fill: egui::Color32::from_rgb(30, 30, 30),
        bg_stroke: egui::Stroke::new(0.0, egui::Color32::from_rgb(230, 230, 230)),
        fg_stroke: egui::Stroke::new(0.0, egui::Color32::from_rgb(230, 230, 230)),
        corner_radius: egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: 0,
            se: 0,
        },
        expansion: 0.0,
    }
}

pub struct Layout {
    pub text_size: f32,
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
            toolbar_height: 40.0,
            tab_width: MinMax {
                min: 40.0,
                max: 200.0,
            },
        }
    }
}
