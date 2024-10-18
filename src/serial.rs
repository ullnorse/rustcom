use anyhow::Result;
use serialport5::{
    available_ports, DataBits, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits,
};

#[derive(Clone, Copy, Debug)]
pub struct SerialSettings {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
    pub flow_control: FlowControl,
}

impl Default for SerialSettings {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            flow_control: FlowControl::None,
        }
    }
}

impl SerialSettings {
    pub fn new(
        port_name: String,
        baudrate: u32,
        data_bits: DataBits,
        stop_bits: StopBits,
        parity: Parity,
        flow_control: FlowControl,
    ) -> Self {
        Self {
            baud_rate: baudrate,
            data_bits,
            stop_bits,
            parity,
            flow_control,
        }
    }
}

pub struct Serial {}

impl Serial {
    pub fn new() -> Self {
        Self {}
    }

    pub fn try_connect(&mut self, port_name: &str, settings: SerialSettings) -> Result<()> {
        let write_port = serialport5::SerialPortBuilder::new()
            .baud_rate(settings.baud_rate)
            .data_bits(settings.data_bits)
            .stop_bits(settings.stop_bits)
            .parity(settings.parity)
            .flow_control(settings.flow_control)
            .read_timeout(Some(std::time::Duration::from_millis(10)))
            .open(port_name)?;

        let read_port = write_port.try_clone()?;

        std::thread::spawn(move || {});

        std::thread::spawn(move || {});

        Ok(())
    }

    pub fn try_disconnect(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn available_ports() -> Vec<String> {
        available_ports()
            .unwrap()
            .iter()
            .map(|serialport_info| serialport_info.port_name.clone())
            .collect()
    }
}
