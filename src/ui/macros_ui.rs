pub fn render_ui(open: bool, window_open: &mut bool, ui: &mut egui::Ui) {
    if open {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Macros");
                if ui.button("Set Macros").clicked() {
                    *window_open = true;
                }

                for i in 0..16 {
                    if ui
                        .button(format!("M{}{}", i + 1, if i < 10 { " " } else { "" }))
                        .clicked()
                    {
                        //TODO: send
                    }
                }

                ui.add_space(ui.available_width());
            });
        });
    }
}
