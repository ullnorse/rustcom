use anyhow::Result;
use crossbeam::channel::{unbounded, Receiver, Sender};
use serialport5::{ClearBuffer, DataBits, FlowControl, Parity, SerialPortBuilder, StopBits};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
    rx_thread_running: Arc<AtomicBool>,
    tx_thread_running: Arc<AtomicBool>,

    tx_channel: (Sender<String>, Receiver<String>),
    rx_channel: (Sender<String>, Receiver<String>),

    open: bool,
}

impl Default for Serial {
    fn default() -> Self {
        Self {
            rx_thread_running: Arc::new(AtomicBool::new(false)),
            tx_thread_running: Arc::new(AtomicBool::new(false)),
            tx_channel: unbounded(),
            rx_channel: unbounded(),
            open: false,
        }
    }
}

impl Serial {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_open(&mut self, port: &str, settings: SerialSettings) -> Result<()> {
        let mut write_port = SerialPortBuilder::new()
            .baud_rate(settings.baud_rate)
            .data_bits(settings.data_bits)
            .stop_bits(settings.stop_bits)
            .parity(settings.parity)
            .flow_control(settings.flow_control)
            .read_timeout(Some(std::time::Duration::from_millis(50)))
            .open(port)?;

        self.open = true;

        write_port.clear(ClearBuffer::All)?;

        let mut read_port = write_port.try_clone()?;
        let mut serial_buf: Vec<u8> = vec![0; 1000];

        let (rx_sender, _) = self.rx_channel.clone();
        let (_, tx_receiver) = self.tx_channel.clone();

        let rx_thread_running = self.rx_thread_running.clone();

        std::thread::spawn(move || {
            rx_thread_running.store(true, Ordering::Relaxed);

            while rx_thread_running.load(Ordering::Relaxed) {
                match read_port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        rx_sender
                            .send(String::from_utf8_lossy(&serial_buf[..t]).to_string())
                            .unwrap();
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        });

        let tx_thread_running = self.tx_thread_running.clone();

        std::thread::spawn(move || {
            tx_thread_running.store(true, Ordering::Relaxed);

            while tx_thread_running.load(Ordering::Relaxed) {
                if let Ok(s) = tx_receiver.try_recv() {
                    write_port.write_all(s.as_bytes()).unwrap();
                }
            }
        });

        Ok(())
    }

    pub fn close(&mut self) {
        self.tx_thread_running.store(false, Ordering::Relaxed);
        self.rx_thread_running.store(false, Ordering::Relaxed);

        self.open = false;
    }

    pub fn send(&self, data: &str) {
        if self.is_open() {
            self.tx_channel.0.send(data.to_string()).unwrap();
        }
    }

    pub fn try_recv(&self) -> Option<String> {
        if self.is_open() {
            return self.rx_channel.1.try_recv().ok();
        }

        None
    }

    pub fn available_ports() -> Vec<String> {
        serialport::available_ports()
            .unwrap()
            .iter()
            .map(|serialport_info| serialport_info.port_name.clone())
            .collect()
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}
