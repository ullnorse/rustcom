use eframe::egui::{self, ComboBox, Response, Widget};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum Protocol {
    #[default]
    Plain,
    XModem,
    YModem,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct FileProtocolPicker<'a> {
    width: f32,
    protocol: &'a mut Protocol,
}

impl<'a> FileProtocolPicker<'a> {
    const PROTOCOLS: [Protocol; 3] = [Protocol::Plain, Protocol::XModem, Protocol::YModem];
    const ID: &'static str = "protocol";

    #[allow(dead_code)]
    pub fn new(width: f32, protocol: &'a mut Protocol) -> Self {
        Self {
            width,
            protocol,
        }
    }
}

impl<'a> Widget for FileProtocolPicker<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ComboBox::from_id_source(Self::ID)
            .selected_text(self.protocol.to_string())
            .width(self.width)
            .show_ui(ui, |ui| {
                for protocol in Self::PROTOCOLS {
                    ui.selectable_value(self.protocol, protocol, protocol.to_string());
                }
            })
            .response
    }
}
