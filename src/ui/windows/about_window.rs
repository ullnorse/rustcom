use egui::{Align2, RichText, Vec2};

pub fn render_window(open: &mut bool, ctx: &egui::Context) {
    egui::Window::new("About")
        .resizable(false)
        .collapsible(false)
        .open(open)
        .anchor(Align2::CENTER_CENTER, Vec2::default())
        .fixed_size([360.0, 240.0])
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 8.0;
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("rustcom")
                        .color(ui.style().visuals.widgets.active.bg_fill)
                        .heading(),
                );
                ui.label("© 2025 Aleksa Jonić - MIT OR Apache-2.0");
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
        });
}
