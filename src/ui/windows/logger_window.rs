use crate::logger::LOGGER;
use log::Level;

pub fn show(open: &mut bool, ctx: &egui::Context) {
    egui::Window::new("Log")
        .resizable(false)
        .open(open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    LOGGER.clear_logs();
                }

                let levels = [
                    Level::Error,
                    Level::Warn,
                    Level::Info,
                    Level::Debug,
                    Level::Trace,
                ];

                let mut log_level = LOGGER.get_level().unwrap_or(Level::max());

                egui::ComboBox::from_label("Log level")
                    .selected_text(format!("{:?}", log_level))
                    .show_ui(ui, |ui| {
                        for level in levels {
                            if ui
                                .selectable_value(&mut log_level, level, level.as_str())
                                .changed()
                            {
                                LOGGER.set_level(log_level);
                            }
                        }
                    });
            });

            let selectable_text = |ui: &mut egui::Ui, mut text: &str| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut text));
                });
            };

            ui.group(|ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        selectable_text(ui, &LOGGER.get_logs().unwrap_or_default());
                    });
            });
        });
}
