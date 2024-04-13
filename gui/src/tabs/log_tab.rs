use base::messages::{self, Message};
use eframe::egui::{Frame, ScrollArea};

use crate::tabs::Tab;
use crate::App;
pub struct LogTab;

impl Tab for LogTab {
    fn title(&self) -> &str {
        "Log"
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, app: &mut App) {
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
                        messages::send(Message::ClearLogText);
                    }

                    ui.label(app.log_text.clone());

                    ui.allocate_space(ui.available_size());
                });
        });
    }
}