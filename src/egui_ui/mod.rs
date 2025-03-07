use eframe::egui;

pub fn start_browser() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Oliver Explorer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<OliverExplorer>::default())
        }),
    )
}

struct OliverExplorer {
    name: String,
    age: u32,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        Self {
            name: "Oliver".to_owned(),
            age: 27,
        }
    }
}

impl eframe::App for OliverExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.image(egui::include_image!("../../assets/tomato.png"));
        });
    }
}
