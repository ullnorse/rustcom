use egui::Ui;
use super::App;
use super::PortSettingsWindow;

pub fn render_settings_tab(app: &mut App, ui: &mut Ui) {
    ui.add_enabled(!app.device_connected, PortSettingsWindow::new(
        &mut app.current_serial_device,
        &app.serial_devices ,
        &mut app.serial_config.baudrate,
        &mut app.serial_config.char_size,
        &mut app.serial_config.stop_bits,
        &mut app.serial_config.parity,
        &mut app.serial_config.flow_control,
        &mut app.local_echo));
}