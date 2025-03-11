use anyhow::Result;
use crossbeam::channel::{unbounded, Receiver, Sender};
use serialport5::{ClearBuffer, SerialPortBuilder};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    #[default]
    Eight,
}

impl From<DataBits> for serialport5::DataBits {
    fn from(value: DataBits) -> Self {
        use DataBits::*;
        match value {
            Five => serialport5::DataBits::Five,
            Six => serialport5::DataBits::Six,
            Seven => serialport5::DataBits::Seven,
            Eight => serialport5::DataBits::Eight,
        }
    }
}

impl std::fmt::Display for DataBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DataBits::*;
        write!(
            f,
            "{}",
            match self {
                Five => "5",
                Six => "6",
                Seven => "7",
                Eight => "8",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum FlowControl {
    #[default]
    None,
    Software,
    Hardware,
}

impl From<FlowControl> for serialport5::FlowControl {
    fn from(value: FlowControl) -> Self {
        use FlowControl::*;
        match value {
            None => serialport5::FlowControl::None,
            Software => serialport5::FlowControl::Software,
            Hardware => serialport5::FlowControl::Hardware,
        }
    }
}

impl std::fmt::Display for FlowControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlowControl::None => "None",
                FlowControl::Software => "Software",
                FlowControl::Hardware => "Hardware",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum StopBits {
    #[default]
    One,
    Two,
}

impl From<StopBits> for serialport5::StopBits {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::One => serialport5::StopBits::One,
            StopBits::Two => serialport5::StopBits::Two,
        }
    }
}

impl std::fmt::Display for StopBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StopBits::One => "One",
                StopBits::Two => "Two",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Parity {
    #[default]
    None,
    Odd,
    Even,
}

impl From<Parity> for serialport5::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => serialport5::Parity::None,
            Parity::Odd => serialport5::Parity::Odd,
            Parity::Even => serialport5::Parity::Even,
        }
    }
}

impl std::fmt::Display for Parity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Parity::None => "None",
                Parity::Odd => "Odd",
                Parity::Even => "Even",
            }
        )
    }
}

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
            baud_rate: 115_200,
            data_bits: DataBits::default(),
            stop_bits: StopBits::default(),
            parity: Parity::default(),
            flow_control: FlowControl::default(),
        }
    }
}

impl SerialSettings {
    pub fn new(
        baud_rate: u32,
        data_bits: DataBits,
        stop_bits: StopBits,
        parity: Parity,
        flow_control: FlowControl,
    ) -> Self {
        Self {
            baud_rate,
            data_bits,
            stop_bits,
            parity,
            flow_control,
        }
    }
}

pub struct Serial {
    rx_thread_running: Arc<AtomicBool>,

    // from user to serial
    tx_channel: (Sender<Option<String>>, Receiver<Option<String>>),

    // from serial to user
    rx_channel: (Sender<String>, Receiver<String>),
}

impl Serial {
    pub fn new(port: &str, settings: SerialSettings) -> Result<Self> {
        let mut write_port = SerialPortBuilder::new()
            .baud_rate(settings.baud_rate)
            .data_bits(settings.data_bits.into())
            .stop_bits(settings.stop_bits.into())
            .parity(settings.parity.into())
            .flow_control(settings.flow_control.into())
            .read_timeout(Some(std::time::Duration::from_millis(200)))
            .open(port)?;

        write_port.clear(ClearBuffer::All)?;

        let mut read_port = write_port.try_clone()?;
        let mut serial_buf: Vec<u8> = vec![0; 1000];

        let rx_channel = unbounded::<String>();
        let tx_channel = unbounded::<Option<String>>();

        let rx_thread_running = Arc::new(AtomicBool::new(false));
        let rx_thread_running_clone = rx_thread_running.clone();

        let rx_sender = rx_channel.0.clone();

        std::thread::spawn(move || -> Result<()> {
            rx_thread_running_clone.store(true, Ordering::Relaxed);

            while rx_thread_running_clone.load(Ordering::Relaxed) {
                match read_port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        rx_sender.send(String::from_utf8_lossy(&serial_buf[..t]).to_string())?;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{e:?}"),
                }
            }

            Ok(())
        });

        let tx_receiver = tx_channel.1.clone();

        std::thread::spawn(move || -> Result<()> {
            while let Ok(Some(s)) = tx_receiver.recv() {
                write_port.write_all(s.as_bytes())?;
            }

            Ok(())
        });

        Ok(Self {
            rx_thread_running,
            tx_channel,
            rx_channel,
        })
    }

    pub fn send(&self, data: &str) -> Result<()> {
        Ok(self.tx_channel.0.send(Some(data.to_string()))?)
    }

    pub fn recv(&self) -> Option<String> {
        self.rx_channel.1.try_recv().ok()
    }

    pub fn available_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .iter()
            .map(|serialport_info| serialport_info.port_name.clone())
            .collect()
    }
}

impl Drop for Serial {
    fn drop(&mut self) {
        self.rx_thread_running.store(false, Ordering::Relaxed);
        self.tx_channel.0.send(None).ok();
    }
}
