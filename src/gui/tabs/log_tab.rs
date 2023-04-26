use super::Tab;

use egui::{Frame, ScrollArea, Label};

use crate::logger::Entry;
use crate::gui::Message;

use log::{info, error};

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
                        if ui.button("add log").clicked() {
                            app.do_update(Message::Log(Entry {
                                timestamp: "11:26".to_string(),
                                level: "INFO".to_string(),
                                target: "target".to_string(),
                                args: "1 2 3 4".to_string()
                            }));
                        }

                        if ui.button("log test").clicked() {
                            error!("aleksa je car");
                        }

                        ui.add(Label::new(app.log.clone()));

                        ui.allocate_space(ui.available_size());
                    });
            });
    }

    fn title(&self) -> &str {
        "Log"
    }
}
