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
            baud_rate: 115_200,
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

    // from user to serial
    tx_channel: (Sender<Option<String>>, Receiver<Option<String>>),

    // from serial to user
    rx_channel: (Sender<String>, Receiver<String>),
}

impl Default for Serial {
    fn default() -> Self {
        Self {
            rx_thread_running: Arc::new(AtomicBool::new(false)),
            tx_channel: unbounded(),
            rx_channel: unbounded(),
        }
    }
}

impl Serial {
    pub fn new(port: &str, settings: SerialSettings) -> Result<Self> {
        let mut write_port = SerialPortBuilder::new()
            .baud_rate(0)
            .data_bits(settings.data_bits)
            .stop_bits(settings.stop_bits)
            .parity(settings.parity)
            .flow_control(settings.flow_control)
            .read_timeout(Some(std::time::Duration::from_millis(50)))
            .open(port)?;

        write_port.clear(ClearBuffer::All)?;

        let mut read_port = write_port.try_clone()?;
        let mut serial_buf: Vec<u8> = vec![0; 1000];

        let rx_channel = unbounded::<String>();
        let tx_channel = unbounded::<Option<String>>();

        let rx_thread_running = Arc::new(AtomicBool::new(false));

        let rx_thread_running_clone = rx_thread_running.clone();

        let rx_sender = rx_channel.0.clone();

        std::thread::spawn(move || {
            rx_thread_running_clone.store(true, Ordering::Relaxed);

            while rx_thread_running_clone.load(Ordering::Relaxed) {
                match read_port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        rx_sender
                            .send(String::from_utf8_lossy(&serial_buf[..t]).to_string())
                            .unwrap(); //TODO: handle unwrap
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{e:?}"),
                }
            }
        });

        let tx_receiver = tx_channel.1.clone();

        std::thread::spawn(move || {
            while let Ok(Some(s)) = tx_receiver.recv() {
                write_port.write_all(s.as_bytes()).unwrap();
            }
        });

        Ok(Self {
            rx_thread_running,
            tx_channel,
            rx_channel,
        })
    }

    pub fn send(&self, data: &str) -> Result<()> {
        self.tx_channel.0.send(Some(data.to_string()))?;
        Ok(())
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
        self.tx_channel.0.send(None).ok(); //TODO: handle result
    }
}
