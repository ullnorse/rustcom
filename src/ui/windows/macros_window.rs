pub fn render_window(open: &mut bool, ctx: &egui::Context) {
    egui::Window::new("Macro")
        .resizable(false)
        .open(open)
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
                                //self.read_config_from_file(path.as_path());

                                if let Ok(_s) = path.into_os_string().into_string() {
                                    //self.config_file = s;
                                }
                            }
                        }
                    }

                    if ui
                        .add_sized(button_size, egui::Button::new("Save"))
                        .clicked()
                    {
                        if let Some(dir) = directories::BaseDirs::new() {
                            if let Some(_path) = rfd::FileDialog::new()
                                .set_title("Save As")
                                .set_directory(dir.home_dir())
                                .save_file()
                            {
                                println!("Saving file to {:?}", _path);
                                // if let Err(e) = self.save_config_to_file(path.as_path()) {
                                //     error!("Couldn't save file: {e:?}");
                                // }
                            }
                        }
                    }

                    //ui.label(&self.config_file);
                });

                for i in 0..16 {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        //let repeat = &mut self.macros[i].repeat;

                        let mut checked = false;
                        if ui.checkbox(&mut checked, "").changed() {
                            //repeat_changes.push(i);
                        }

                        let mut delay: u32 = 0;
                        spinbox(ui, &mut delay, 0, u32::MAX, 10);

                        if ui
                            .add_sized(
                                egui::vec2(50.0, 20.0),
                                egui::Button::new(format!("M{}", i + 1)),
                            )
                            .clicked()
                        {
                            // TODO: send
                        }

                        let mut macro_name = String::new();
                        ui.text_edit_singleline(&mut macro_name);
                    });
                }
            });
        });
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
