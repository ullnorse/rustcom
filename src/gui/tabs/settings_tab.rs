use egui::Ui;
use crate::gui::widgets::port_settings::PortSettingsWindow;
use crate::gui::Message;
use super::App;
use super::Tab;

pub struct SettingsTab;

impl Tab for SettingsTab {
    fn show_ui(&self, app: &mut App, ui: &mut Ui) {
        if ui.button("Refresh").clicked() {
            app.do_update(Message::RefreshSerialDevices);
        }

        ui.add_enabled(!app.device_connected,
            PortSettingsWindow::new(
                &mut app.current_serial_device,
                &app.serial_devices ,
                &mut app.serial_config.baudrate,
                &mut app.serial_config.char_size,
                &mut app.serial_config.stop_bits,
                &mut app.serial_config.parity,
                &mut app.serial_config.flow_control,
                &mut app.local_echo
            ));
    }

    fn title(&self) -> &str {
        "Settings"
    }
}
