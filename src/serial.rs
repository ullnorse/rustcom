use anyhow::{Context, Result, anyhow};
use crossbeam::channel::{Receiver, Sender, unbounded};
use log::error;
use serialport::ClearBuffer;
use std::io::{Read, Write};
use std::mem;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const SERIAL_READ_TIMEOUT_MS: u64 = 50;
const SERIAL_READ_BUFFER_SIZE: usize = 64;

pub enum SerialMsg {
    Str(String),
    Close,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    #[default]
    Eight,
}

impl From<DataBits> for serialport::DataBits {
    fn from(value: DataBits) -> Self {
        use DataBits::*;
        match value {
            Five => serialport::DataBits::Five,
            Six => serialport::DataBits::Six,
            Seven => serialport::DataBits::Seven,
            Eight => serialport::DataBits::Eight,
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

impl From<FlowControl> for serialport::FlowControl {
    fn from(value: FlowControl) -> Self {
        use FlowControl::*;
        match value {
            None => serialport::FlowControl::None,
            Software => serialport::FlowControl::Software,
            Hardware => serialport::FlowControl::Hardware,
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

impl From<StopBits> for serialport::StopBits {
    fn from(value: StopBits) -> Self {
        match value {
            StopBits::One => serialport::StopBits::One,
            StopBits::Two => serialport::StopBits::Two,
        }
    }
}

impl std::fmt::Display for StopBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StopBits::One => "1",
                StopBits::Two => "2",
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

impl From<Parity> for serialport::Parity {
    fn from(value: Parity) -> Self {
        match value {
            Parity::None => serialport::Parity::None,
            Parity::Odd => serialport::Parity::Odd,
            Parity::Even => serialport::Parity::Even,
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

#[derive(Clone, Debug)]
pub struct SerialSettings {
    pub port: String,
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
    pub flow_control: FlowControl,
}

impl Default for SerialSettings {
    fn default() -> Self {
        Self {
            port: String::new(),
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
        port: String,
        baud_rate: u32,
        data_bits: DataBits,
        stop_bits: StopBits,
        parity: Parity,
        flow_control: FlowControl,
    ) -> Self {
        Self {
            port,
            baud_rate,
            data_bits,
            stop_bits,
            parity,
            flow_control,
        }
    }
}

pub struct SerialMainState {
    rx_thread_running: Arc<AtomicBool>,
    tx_sender: Sender<SerialMsg>,
    join_handles: Option<(JoinHandle<()>, JoinHandle<()>)>,
    rx_receiver: Receiver<String>,
}

impl SerialMainState {
    pub fn new(settings: SerialSettings) -> Result<Self> {
        let mut write_port = serialport::new(settings.port, 0)
            .baud_rate(settings.baud_rate)
            .data_bits(settings.data_bits.into())
            .stop_bits(settings.stop_bits.into())
            .parity(settings.parity.into())
            .flow_control(settings.flow_control.into())
            .timeout(Duration::from_millis(SERIAL_READ_TIMEOUT_MS))
            .open()?;

        write_port.clear(ClearBuffer::All)?;

        let mut read_port = write_port.try_clone()?;

        let (rx_sender, rx_receiver) = unbounded::<String>();
        let (tx_sender, tx_receiver) = unbounded::<SerialMsg>();

        let rx_thread_running = Arc::new(AtomicBool::new(true));
        let rx_thread_running_clone = rx_thread_running.clone();

        let rx_join_handle = std::thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut temp_buf = [0; SERIAL_READ_BUFFER_SIZE];

            while rx_thread_running_clone.load(Ordering::SeqCst) {
                match read_port.read(&mut temp_buf) {
                    Ok(n) => {
                        buffer.extend_from_slice(&temp_buf[..n]);

                        while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                            let line = buffer.drain(..=pos).collect::<Vec<u8>>();
                            let string = String::from_utf8_lossy(&line).to_string();
                            if let Err(e) = rx_sender.send(string) {
                                error!("Error receiving serial data: {e}");
                            }
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        if !buffer.is_empty()
                            && let Ok(s) = String::from_utf8(mem::take(&mut buffer))
                            && let Err(e) = rx_sender.send(s)
                        {
                            error!("Error receiving serial data on timeout: {e}");
                        }
                    }
                    Err(e) => {
                        error!("RX thread error: {e:?}");
                        return;
                    }
                }
            }
        });

        let tx_join_handle = std::thread::spawn(move || {
            while let Ok(msg) = tx_receiver.recv() {
                match msg {
                    SerialMsg::Str(s) => {
                        if let Err(e) = write_port.write_all(s.as_bytes()) {
                            error!("Error writing to serial port: {e}");
                        }
                    }
                    SerialMsg::Close => break,
                }
            }
        });

        Ok(Self {
            rx_thread_running,
            tx_sender,
            join_handles: Some((rx_join_handle, tx_join_handle)),
            rx_receiver,
        })
    }

    pub fn send(&self, msg: SerialMsg) -> Result<()> {
        self.tx_sender
            .send(msg)
            .context("Failed to send serial message")
    }

    pub fn recv(&self) -> Result<String> {
        Ok(self.rx_receiver.try_recv()?)
    }

    pub fn available_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap_or_default()
            .iter()
            .map(|serialport_info| serialport_info.port_name.clone())
            .collect()
    }

    pub fn close(mut self) -> Result<()> {
        self.close_internal()
            .map_err(|e| anyhow!("Unable to join worker threads: {e:?}"))
    }

    fn close_internal(&mut self) -> thread::Result<()> {
        if let Some((rx_handle, tx_handle)) = self.join_handles.take() {
            self.rx_thread_running.store(false, Ordering::SeqCst);
            let _ = self.tx_sender.send(SerialMsg::Close);
            rx_handle.join()?;
            tx_handle.join()?;
        }
        Ok(())
    }
}

impl Drop for SerialMainState {
    fn drop(&mut self) {
        if let Err(e) = self.close_internal() {
            error!("Unable to close serial port: {e:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_bits_display() {
        assert_eq!(DataBits::Five.to_string(), "5");
        assert_eq!(DataBits::Six.to_string(), "6");
        assert_eq!(DataBits::Seven.to_string(), "7");
        assert_eq!(DataBits::Eight.to_string(), "8");
    }

    #[test]
    fn test_stop_bits_display() {
        assert_eq!(StopBits::One.to_string(), "1");
        assert_eq!(StopBits::Two.to_string(), "2");
    }

    #[test]
    fn test_parity_display() {
        assert_eq!(Parity::None.to_string(), "None");
        assert_eq!(Parity::Odd.to_string(), "Odd");
        assert_eq!(Parity::Even.to_string(), "Even");
    }

    #[test]
    fn test_flow_control_display() {
        assert_eq!(FlowControl::None.to_string(), "None");
        assert_eq!(FlowControl::Software.to_string(), "Software");
        assert_eq!(FlowControl::Hardware.to_string(), "Hardware");
    }

    #[test]
    fn test_data_bits_conversion() {
        assert_eq!(
            serialport::DataBits::from(DataBits::Five),
            serialport::DataBits::Five
        );
        assert_eq!(
            serialport::DataBits::from(DataBits::Eight),
            serialport::DataBits::Eight
        );
    }

    #[test]
    fn test_parity_conversion() {
        assert_eq!(
            serialport::Parity::from(Parity::None),
            serialport::Parity::None
        );
        assert_eq!(
            serialport::Parity::from(Parity::Even),
            serialport::Parity::Even
        );
    }

    #[test]
    fn test_flow_control_conversion() {
        assert_eq!(
            serialport::FlowControl::from(FlowControl::None),
            serialport::FlowControl::None
        );
        assert_eq!(
            serialport::FlowControl::from(FlowControl::Hardware),
            serialport::FlowControl::Hardware
        );
    }

    #[test]
    fn test_stop_bits_conversion() {
        assert_eq!(
            serialport::StopBits::from(StopBits::One),
            serialport::StopBits::One
        );
        assert_eq!(
            serialport::StopBits::from(StopBits::Two),
            serialport::StopBits::Two
        );
    }
}
