use crate::ui::config;
use crate::ui::types;
use eframe::egui;

// Tab bar component
pub struct TabBar<'a> {
    pub active_tab: usize,
    pub tabs: &'a mut Vec<types::Tab>,
}

impl<'a> TabBar<'a> {
    // Create a new tab bar
    pub fn new(active_tab: usize, tabs: &'a mut Vec<types::Tab>) -> Self {
        Self { active_tab, tabs }
    }

    // Show the tab bar returns the index of the active tab
    pub fn show(&mut self, ui: &mut egui::Ui) -> usize {
        let original_spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing.x = config::TAB_SPACING;
        ui.horizontal(|ui| {
            // Tab bar height
            ui.set_min_height(config::TAB_BAR_HEIGHT);

            // Tab items
            let tab_width = self.calculate_tab_width(ui);
            for (index, tab) in self.tabs.iter().enumerate() {
                let tab_item = TabItem::new(tab, self.active_tab == index, tab_width).show(ui);
                if tab_item.clicked() {
                    self.active_tab = index;
                }
            }

            // New tab button
            let plus_button = plus_button(ui);
            if plus_button.clicked() {
                self.tabs.push(types::Tab::new(config::DEFAULT_URL));
                self.active_tab = self.tabs.len() - 1;
            }
        });
        ui.spacing_mut().item_spacing = original_spacing;
        self.active_tab
    }

    // Calculate tab width based on available space
    fn calculate_tab_width(&self, ui: &mut egui::Ui) -> f32 {
        let tab_count = self.tabs.len() as f32;
        let available_width = ui.available_width();
        let plus_button_width = ui.available_size().y;
        let spacing_width = tab_count * config::TAB_SPACING;
        let width_per_tab = (available_width - plus_button_width - spacing_width) / tab_count;
        width_per_tab.min(config::MAX_TAB_WIDTH)
    }
}

// Tab item component
pub struct TabItem<'a> {
    pub tab: &'a types::Tab,
    pub is_active: bool,
    pub width: f32,
}

impl<'a> TabItem<'a> {
    // Create a new tab item
    pub fn new(tab: &'a types::Tab, is_active: bool, width: f32) -> Self {
        Self {
            tab,
            is_active,
            width,
        }
    }

    // Show the tab item
    pub fn show(&self, ui: &mut egui::Ui) -> egui::Response {
        let url = self.tab.get_url();
        let tab_name = url.clone();

        // Get the background fill and stroke color for the tab
        let (bg_fill, stroke_color) = if self.is_active {
            (
                ui.style().visuals.selection.bg_fill,
                ui.style().visuals.selection.stroke.color,
            )
        } else {
            (
                ui.style().visuals.widgets.inactive.bg_fill,
                ui.style().visuals.widgets.inactive.text_color(),
            )
        };

        // Create a frame for the tab with fixed width
        let frame = egui::Frame::new().fill(bg_fill);
        frame
            .show(ui, |ui| {
                ui.set_width(self.width);
                ui.add(
                    egui::Label::new(egui::RichText::new(tab_name).color(stroke_color)).truncate(),
                )
            })
            .response
            .interact(egui::Sense::click())
    }
}

// Plus button component
fn plus_button(ui: &mut egui::Ui) -> egui::Response {
    ui.add_sized(
        [ui.available_size().y, ui.available_size().y],
        egui::Button::new("+"),
    )
}
