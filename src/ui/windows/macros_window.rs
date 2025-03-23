use crate::{app::App, macros::Macros, serial::SerialMsg};
use log::error;

impl App {
    pub fn show_macros_window(&mut self, ctx: &egui::Context) {
        let mut macros_window_open = self.macros_window_open;

        egui::Window::new("Macro")
            .resizable(false)
            .open(&mut macros_window_open)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let button_size = egui::vec2(60.0, 30.0);

                        if ui
                            .add_sized(button_size, egui::Button::new("Load"))
                            .clicked()
                        {
                            if let Some(dir) = directories::BaseDirs::new() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_title("Open")
                                    .set_directory(dir.home_dir())
                                    .pick_file()
                                {
                                    self.macros.read_config_from_file(&path);

                                    if let Ok(s) = path.into_os_string().into_string() {
                                        self.macros.set_config_file(s);
                                    }
                                }
                            }
                        }

                        if ui
                            .add_sized(button_size, egui::Button::new("Save"))
                            .clicked()
                        {
                            if let Some(dir) = directories::BaseDirs::new() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_title("Save As")
                                    .set_directory(dir.home_dir())
                                    .save_file()
                                {
                                    if let Err(e) = self.macros.save_config_to_file(&path) {
                                        error!("Couldn't save file: {e:?}");
                                    }
                                }
                            }
                        }

                        ui.label(self.macros.get_config_file());
                    });

                    let mut updated_macros = Vec::new();

                    for i in 0..Macros::NUM_OF_MACROS {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            if let Some(m) = self.macros.get_macro(i) {
                                let mut macro_copy = m.clone();

                                ui.checkbox(&mut macro_copy.repeat, "");
                                spinbox(ui, &mut macro_copy.delay, 0, u32::MAX, 10);

                                if ui
                                    .add_sized(
                                        egui::vec2(50.0, 20.0),
                                        egui::Button::new(format!("M{}", i + 1)),
                                    )
                                    .clicked()
                                {
                                    let text = macro_copy.text.clone();

                                    if let Err(e) = self.serial_send(SerialMsg::Str(text)) {
                                        error!("Couldn't send macro text: {e:?}");
                                    }
                                }

                                ui.text_edit_singleline(&mut macro_copy.text);

                                updated_macros.push((i, macro_copy));
                            }
                        });
                    }

                    for (i, m) in updated_macros {
                        self.macros.set_macro(i, m);
                    }
                });
            });

        self.macros_window_open = macros_window_open;
    }
}

fn spinbox(ui: &mut egui::Ui, value: &mut u32, min: u32, max: u32, step: u32) {
    ui.horizontal(|ui| {
        if ui.add(egui::Button::new("+")).clicked() {
            *value = (*value + step).min(max);
        }

        ui.add(
            egui::DragValue::new(value)
                .range(0..=u32::MAX)
                .speed(step as f64),
        );

        if ui.add(egui::Button::new("-")).clicked() {
            *value = value.saturating_sub(step).max(min);
        }
    });
}
