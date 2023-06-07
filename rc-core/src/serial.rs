use anyhow::Result;
use serial2::{SerialPort, CharSize, FlowControl, Parity, StopBits, IntoSettings};
use flume::{unbounded, Receiver, Sender};
use std::{sync::Arc, thread, io::Read};

#[derive(Debug, Clone)]
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

pub struct Serial {
    transmit_state_channel: (Sender<()>, Receiver<()>),
    receive_state_channel: (Sender<()>, Receiver<()>),
    data_channel: (Sender<String>, Receiver<String>),
    output_channel: (Sender<String>, Receiver<String>),
}

impl Default for Serial {
    fn default() -> Self {
        Self {
            transmit_state_channel: unbounded(),
            receive_state_channel: unbounded(),
            data_channel: unbounded(),
            output_channel: unbounded(),
        }
    }
}

impl Serial {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn available_ports() -> Result<Vec<String>> {
        let ports = serialport::available_ports()?;
        let names: Vec<String> = ports.iter().map(|port| port.port_name.clone()).collect();
        Ok(names)
    }

    pub fn start(&self, port_name: &str, config: SerialConfig) -> Result<()> {
        let (_, transmit_state_channel) = self.transmit_state_channel.clone();
        let (_, receive_state_channel) = self.receive_state_channel.clone();

        let (data_sender, _) = self.data_channel.clone();
        let (_, output_receiver) = self.output_channel.clone();

        let mut port = SerialPort::open(port_name, config)?;
        port.set_read_timeout(std::time::Duration::from_millis(10))?;
        port.discard_buffers()?;

        Self::clear_buffer(&mut port);

        let shared_port = Arc::new(port);

        let receive_port = shared_port.clone();

        thread::spawn(move || {
            let mut buf = [0u8; 1000];

            loop {
                if receive_state_channel.try_recv().is_ok() {
                    break;
                }

                match receive_port.read(buf.as_mut_slice()) {
                    Ok(read_bytes) => {
                        let s = String::from_utf8_lossy(&buf[..read_bytes]);
                        data_sender.send(s.to_string()).ok();
                    },
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{e:?}"),
                }
            }
        });

        let transmit_port = shared_port;

        thread::spawn(move || {
            loop {
                if transmit_state_channel.try_recv().is_ok() {
                    break;
                }

                if let Ok(s) = output_receiver.recv_timeout(std::time::Duration::from_millis(100)) {
                    if let Ok(_size) = transmit_port.write(s.as_bytes()) {

                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.transmit_state_channel.0.send(())?;
        self.receive_state_channel.0.send(())?;

        Ok(())
    }

    pub fn send(&self, data: &str) {
        self.output_channel.0.send(data.to_string()).unwrap();
    }

    pub fn try_recv(&self) -> Option<String> {
        self.data_channel.1.try_recv().ok()
    }

    fn clear_buffer(port: &mut SerialPort) {
        let mut buf = [0u8; 10];

        while port.read_exact(&mut buf).is_ok() {

        }
    }
}

