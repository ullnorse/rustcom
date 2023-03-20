use serial2::{CharSize, FlowControl, Parity, StopBits, SerialPort, IntoSettings};

#[derive(Debug)]
pub struct SerialConfig {
    pub baudrate: u32,
    pub char_size: CharSize,
    pub parity: Parity,
    pub flow_control: FlowControl,
    pub stop_bits: StopBits,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            baudrate: 115200,
            char_size: CharSize::Bits8,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
        }
    }
}

impl IntoSettings for SerialConfig {
    fn apply_to_settings(self, settings: &mut serial2::Settings) -> std::io::Result<()> {
        settings.set_baud_rate(self.baudrate)?;
        settings.set_char_size(self.char_size);
        settings.set_stop_bits(self.stop_bits);
        settings.set_parity(self.parity);
        settings.set_flow_control(self.flow_control);
        Ok(())
    }
}