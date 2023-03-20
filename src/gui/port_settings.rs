use egui::{ComboBox, Grid, Response, Widget, TextBuffer};
use serial2::{Parity, CharSize, FlowControl, StopBits, COMMON_BAUD_RATES};

pub struct PortSettingsWindow<'a> {
    serial_devices: &'a Vec<String>,
    serial_device: &'a mut String,
    baudrate: &'a mut u32,
    selected_char_size: &'a mut CharSize,
    selected_stop_bits: &'a mut StopBits,
    selected_parity: &'a mut Parity,
    selected_flow_control: &'a mut FlowControl,
    local_echo: &'a mut bool,
}

impl<'a> PortSettingsWindow<'a> {
    const PARITY: [Parity; 3] = [Parity::None, Parity::Even, Parity::Odd];
    const CHAR_SIZE: [CharSize; 4] = [CharSize::Bits8, CharSize::Bits7, CharSize::Bits6, CharSize::Bits5];
    const FLOW_CONTROL: [FlowControl; 3] = [FlowControl::None, FlowControl::RtsCts, FlowControl::XonXoff];
    const STOP_BITS: [StopBits; 2] = [StopBits::One, StopBits::Two];

    pub fn new(
        serial_device: &'a mut String,
        serial_devices: &'a Vec<String>,
        baudrate: &'a mut u32,
        selected_char_size: &'a mut CharSize,
        selected_stop_bits: &'a mut StopBits,
        selected_parity: &'a mut Parity,
        selected_flow_control: &'a mut FlowControl,
        local_echo: &'a mut bool,
    ) -> Self {
        Self {
            serial_device,
            serial_devices,
            baudrate,
            selected_char_size,
            selected_stop_bits,
            selected_parity,
            selected_flow_control,
            local_echo,
        }
    }
}

impl<'a> Widget for PortSettingsWindow<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        Grid::new("grid")
            .show(ui, |ui| {
                ui.label("Device");
                ComboBox::from_id_source("device")
                    .selected_text(self.serial_device.to_string())
                    .show_ui(ui, |ui| {
                        for device in self.serial_devices {
                            ui.selectable_value(self.serial_device, device.clone(), device.clone());
                        }
                    });
                ui.end_row();

                ui.label("Baud Rate");
                ComboBox::from_id_source("baudrate")
                    .selected_text(format!("{}", *self.baudrate as i32))
                    .show_ui(ui, |ui| {
                        for baudrate in COMMON_BAUD_RATES {
                            ui.selectable_value(self.baudrate, *baudrate, baudrate.to_string());
                        }
                    });
                ui.end_row();

                ui.label("Char bits");
                ComboBox::from_id_source("char_bits")
                    .selected_text(format!("{:?}", self.selected_char_size).char_range(4..5))
                    .show_ui(ui, |ui| {
                        for char_size in Self::CHAR_SIZE {
                            ui.selectable_value(
                                self.selected_char_size,
                                char_size,
                                format!("{char_size:?}").char_range(4..5),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Stop Bits");
                ComboBox::from_id_source("stop_bits")
                    .selected_text(format!("{:?}", self.selected_stop_bits))
                    .show_ui(ui, |ui| {
                        for stop_bits in Self::STOP_BITS {
                            ui.selectable_value(
                                self.selected_stop_bits,
                                stop_bits,
                                format!("{stop_bits:?}"),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Parity");
                ComboBox::from_id_source("parity")
                    .selected_text(format!("{:?}", self.selected_parity))
                    .show_ui(ui, |ui| {
                        for parity in Self::PARITY {
                            ui.selectable_value(
                                self.selected_parity,
                                parity,
                                format!("{parity:?}"),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Flow control");
                ComboBox::from_id_source("flow_control")
                    .selected_text(format!("{:?}", self.selected_flow_control))
                    .show_ui(ui, |ui| {
                        for flow_control in Self::FLOW_CONTROL {
                            ui.selectable_value(
                                self.selected_flow_control,
                                flow_control,
                                format!("{flow_control:?}"),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Local Echo");
                ui.checkbox(self.local_echo, "");
                ui.end_row();
            })
            .response
    }
}
