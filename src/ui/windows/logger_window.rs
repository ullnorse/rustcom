use crate::{app::App, logger::LOGGER};
use eframe::egui::{Align, ComboBox, Context, Layout, ScrollArea, TextEdit, Ui, Window};
use log::Level;

impl App {
    pub fn show_logger_window(&mut self, ctx: &Context) {
        Window::new("Log")
            .resizable(false)
            .open(&mut self.logger_window_open)
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

                    ComboBox::from_label("Log level")
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

                let selectable_text = |ui: &mut Ui, mut text: &str| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.add_sized(ui.available_size(), TextEdit::multiline(&mut text));
                    });
                };

                ui.group(|ui| {
                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            selectable_text(ui, &LOGGER.get_logs().unwrap_or_default());
                        });
                });
            });
    }
}
