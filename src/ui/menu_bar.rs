use crate::app::App;

fn create_menu_item(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: Option<&str>,
    mut callback: impl FnMut(),
) {
    if ui
        .button(format!("{:<30}{}", label, shortcut.unwrap_or_default()))
        .clicked()
    {
        callback();
        ui.close_menu();
    }
}

fn file_menu(app: &mut App, ui: &mut egui::Ui, ctx: &egui::Context) {
    create_menu_item(ui, "Quit", None, || app.quit(ctx));
}

fn edit_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(ui, "Cut", Some("Ctrl+X"), || app.cut().unwrap());
    create_menu_item(ui, "Copy", Some("Ctrl+C"), || app.copy().unwrap());
    create_menu_item(ui, "Paste", Some("Ctrl+V"), || app.paste().unwrap());

    ui.separator();

    create_menu_item(ui, "Clear", Some("Ctrl+L"), || app.output_text.clear());
}

fn help_menu(app: &mut App, ui: &mut egui::Ui) {
    create_menu_item(ui, "Show About", None, || app.about_window_open = true);
    create_menu_item(ui, "Show Log", None, || app.logger_window_open = true);
}

impl App {
    pub fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.style_mut().visuals.button_frame = false;

            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| file_menu(self, ui, ctx));
                ui.menu_button("Edit", |ui| edit_menu(self, ui));
                ui.menu_button("Help", |ui| help_menu(self, ui));
            });
        });
    }
}
