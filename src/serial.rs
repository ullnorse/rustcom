use std::{io::{Read, Write}, primitive};

use anyhow::Result;
use crossbeam::channel::{Sender, Receiver, unbounded};
use serialport5::{
    available_ports, DataBits, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits, ClearBuffer
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

pub struct Serial {
    rx_thread_state_channel: (Sender<()>, Receiver<()>),
    tx_thread_state_channel: (Sender<()>, Receiver<()>),

    tx_channel: (Sender<String>, Receiver<String>),
    rx_channel: (Sender<String>, Receiver<String>),

    connected: bool,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            rx_thread_state_channel: unbounded(),
            tx_thread_state_channel: unbounded(),
            tx_channel: unbounded(),
            rx_channel: unbounded(),
            connected: false,
        }
    }

    pub fn try_connect(&mut self, port_name: &str, settings: SerialSettings) -> Result<()> {
        let mut write_port = serialport5::SerialPortBuilder::new()
            .baud_rate(settings.baud_rate)
            .data_bits(settings.data_bits)
            .stop_bits(settings.stop_bits)
            .parity(settings.parity)
            .flow_control(settings.flow_control)
            .read_timeout(Some(std::time::Duration::from_millis(100)))
            .open("COM16")?;

        self.connected = true;

        write_port.clear(ClearBuffer::All);

        let mut read_port = write_port.try_clone()?;
        let mut serial_buf: Vec<u8> = vec![0; 1000];

        let (_, rx_thread_state_receiver) = self.rx_thread_state_channel.clone();
        let (_, tx_thread_state_receiver) = self.tx_thread_state_channel.clone();

        let (rx_sender, _) = self.rx_channel.clone();
        let (_, tx_receiver) = self.tx_channel.clone();

        std::thread::spawn(move || {
            loop {
                if rx_thread_state_receiver.try_recv().is_ok() {
                    break;
                }

                match read_port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        rx_sender.send(String::from_utf8_lossy(&serial_buf[..t]).to_string());
                    },
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        });

        std::thread::spawn(move || {
            loop {
                if tx_thread_state_receiver.try_recv().is_ok() {
                    break;
                }

                if let Ok(s) = tx_receiver.try_recv() {
                    write_port.write_all(s.as_bytes());
                }
            }
        });

        Ok(())
    }

    pub fn try_disconnect(&mut self) -> Result<()> {
        self.tx_thread_state_channel.0.send(())?;
        self.rx_thread_state_channel.0.send(())?;

        self.connected = false;

        Ok(())
    }

    pub fn send(&self, data: &str) {
        if (self.is_connected()) {
            self.tx_channel.0.send(data.to_string()).unwrap();
        }
    }

    pub fn try_recv(&self) -> Option<String> {
        if (self.is_connected()) {
            return self.rx_channel.1.try_recv().ok();
        }

        None
    }

    pub fn available_ports() -> Vec<String> {
        available_ports()
            .unwrap()
            .iter()
            .map(|serialport_info| serialport_info.port_name.clone())
            .collect()
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
