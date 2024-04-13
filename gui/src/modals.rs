use super::App;
use base::messages;
use eframe::egui::{self, RichText, Layout, Vec2, Align, Align2, Frame};
use super::Message;

impl App {
    pub fn render_about(&self, ctx: &egui::Context) {
        if self.show_about {
            egui::Window::new("About")
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, Vec2::default())
                .fixed_size([360.0, 240.0])
                .frame(Frame::window(&ctx.style()).inner_margin(8.))
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("rustcom").color(ui.style().visuals.widgets.inactive.bg_fill).heading());
                        ui.label("© 2023 Aleksa Jonić - MIT OR Apache-2.0");
                        ui.label(concat!("Version ", env!("CARGO_PKG_VERSION")));
                    });
                    egui::Grid::new("about_box").num_columns(2).show(ui, |ui| {
                        ui.label("GitHub:");
                        if ui.link("https://github.com/ullnorse/rustcom").clicked() {
                            open::that("https://github.com/ullnorse/rustcom").unwrap_or(());
                        }
                        ui.end_row();
                        ui.label("GUI library:");
                        if ui.link("egui").clicked() {
                            open::that("https://github.com/emilk/egui").unwrap_or(());
                        }
                        ui.end_row();
                    });
                    let width = ui.min_size().x;
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(width, ui.min_size().y),
                            Layout::right_to_left(Align::Center),
                            |ui| {
                                if ui.button("OK").clicked() {
                                    messages::send(Message::CloseAbout);
                                }
                                ui.shrink_width_to_current();
                            },
                        );
                    });
                });
        }
    }
}