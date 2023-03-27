use egui::{ComboBox, Grid, TextBuffer, Ui};
use serial2::{Parity, CharSize, FlowControl, StopBits, COMMON_BAUD_RATES};
use crate::gui::Message;
use super::App;
use super::Tab;

pub struct SettingsTab;

impl SettingsTab {
    const PARITY: [Parity; 3] = [Parity::None, Parity::Even, Parity::Odd];
    const CHAR_SIZE: [CharSize; 4] = [CharSize::Bits8, CharSize::Bits7, CharSize::Bits6, CharSize::Bits5];
    const STOP_BITS: [StopBits; 2] = [StopBits::One, StopBits::Two];
    const FLOW_CONTROL: [FlowControl; 3] = [FlowControl::None, FlowControl::RtsCts, FlowControl::XonXoff];
}

impl Tab for SettingsTab {
    fn show_ui(&self, app: &mut App, ui: &mut Ui) {
        ui.set_enabled(!app.device_connected);

        Grid::new("settings_tab_grid").show(ui, |ui| {
            ui.label("Device");
            ComboBox::from_id_source("device")
            .selected_text(app.current_serial_device.clone())
            .show_ui(ui, |ui| {
                for device in &app.serial_devices {
                    ui.selectable_value(&mut app.current_serial_device, device.clone(), device.clone());
                }
            });
            if ui.button("Refresh").clicked() {
                app.do_update(Message::RefreshSerialDevices);
            }
            ui.end_row();

            ui.label("Baud Rate");
            ComboBox::from_id_source("baudrate")
                .selected_text(format!("{}", app.serial_config.baudrate as i32))
                .show_ui(ui, |ui| {
                    for baudrate in COMMON_BAUD_RATES {
                        ui.selectable_value(&mut app.serial_config.baudrate, *baudrate, baudrate.to_string());
                    }
                });
            ui.end_row();

            ui.label("Char bits");
            ComboBox::from_id_source("char_bits")
                .selected_text(format!("{:?}", app.serial_config.char_size).char_range(4..5))
                .show_ui(ui, |ui| {
                    for char_size in Self::CHAR_SIZE {
                        ui.selectable_value(
                            &mut app.serial_config.char_size,
                            char_size,
                            format!("{char_size:?}").char_range(4..5),
                        );
                    }
                });
            ui.end_row();

            ui.label("Stop Bits");
            ComboBox::from_id_source("stop_bits")
                .selected_text(format!("{:?}", app.serial_config.stop_bits))
                .show_ui(ui, |ui| {
                    for stop_bits in Self::STOP_BITS {
                        ui.selectable_value(
                            &mut app.serial_config.stop_bits,
                            stop_bits,
                            format!("{stop_bits:?}"),
                        );
                    }
                });
            ui.end_row();

            ui.label("Parity");
            ComboBox::from_id_source("parity")
                .selected_text(format!("{:?}", app.serial_config.parity))
                .show_ui(ui, |ui| {
                    for parity in Self::PARITY {
                        ui.selectable_value(
                            &mut app.serial_config.parity,
                            parity,
                            format!("{parity:?}"),
                        );
                    }
                });
            ui.end_row();

            ui.label("Flow control");
            ComboBox::from_id_source("flow_control")
                .selected_text(format!("{:?}", app.serial_config.flow_control))
                .show_ui(ui, |ui| {
                    for flow_control in Self::FLOW_CONTROL {
                        ui.selectable_value(
                            &mut app.serial_config.flow_control,
                            flow_control,
                            format!("{flow_control:?}"),
                        );
                    }
                });
            ui.end_row();
        });
    }

    fn title(&self) -> &str {
        "Settings"
    }
}
