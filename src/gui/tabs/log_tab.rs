use super::Tab;

use egui::{Frame, ScrollArea};
use crate::gui::Message;

pub struct LogTab;

impl Tab for LogTab {
    fn show_ui(&self, app: &mut crate::gui::App, ui: &mut egui::Ui) {
        Frame::none()
            .fill(ui.style().visuals.extreme_bg_color)
            .inner_margin(-2.0)
            .outer_margin(0.0)
            .show(ui, |ui| {
                ScrollArea::new([true, true])
                    .auto_shrink([false, true])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        if ui.button("Clear").clicked() {
                            app.do_update(Message::ClearLogText);
                        }

                        ui.label(app.log_text.clone());

                        ui.allocate_space(ui.available_size());
                    });
            });
    }

    fn title(&self) -> &str {
        "Log"
    }
}
