pub mod main_area;
pub mod menu_bar;
pub mod status_bar;
pub mod windows;

pub fn render_combo_box<T>(ui: &mut egui::Ui, label: &str, current_value: &mut T, options: &[T])
where
    T: ToString + Clone + PartialEq,
{
    egui::ComboBox::from_label(label)
        .selected_text(current_value.to_string())
        .show_ui(ui, |ui| {
            for option in options {
                ui.selectable_value(current_value, option.clone(), option.to_string());
            }
        });
}
