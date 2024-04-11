use anyhow::Result;
use clap::Parser;
use serial2::{FlowControl, CharSize, Parity, StopBits};

use crate::serial::SerialConfig;

pub enum AppType {
    Tui,
    Gui,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, help = "Disable gui and start rustcom as a TUI application")]
    no_gui: bool,

    #[arg(short, long, help = "Serial device name")]
    device: Option<String>,

    #[arg(short, long, help = "115200 (default)")]
    baudrate: Option<u32>,

    #[arg(short = 'c', long, value_parser = possible_char_size, help = "Possible values: 5, 6, 7, 8 (default)")]
    char_size: Option<CharSize>,

    #[arg(short, long, value_parser = possible_parity, help = "Possible values: none (default), odd, even")]
    parity: Option<Parity>,

    #[arg(short, long, value_parser = possible_flow_control, help = "Possible values: none (default), xonxoff, rtscts")]
    flow_control: Option<FlowControl>,

    #[arg(short, long, value_parser = possible_stop_bits, help = "Possible values: 1 (default), 2")]
    stop_bits: Option<StopBits>,
}

fn possible_char_size(s: &str) -> Result<CharSize, String> {
    match s {
        "5" => Ok(CharSize::Bits5),
        "6" => Ok(CharSize::Bits6),
        "7" => Ok(CharSize::Bits7),
        "8" => Ok(CharSize::Bits8),
        _ => Err("Possible values: 5, 6, 7, 8 (default)".to_string())
    }
}

fn possible_parity(s: &str) -> Result<Parity, String> {
    match s {
        "none" => Ok(Parity::None),
        "even" => Ok(Parity::Even),
        "odd" => Ok(Parity::Odd),
        _ => Err("Possible values: none (default), odd, even".to_string())
    }
}

fn possible_flow_control(s: &str) -> Result<FlowControl, String> {
    match s {
        "none" => Ok(FlowControl::None),
        "xonxoff" => Ok(FlowControl::XonXoff),
        "rtscts" => Ok(FlowControl::RtsCts),
        _ => Err("Possible values: none (default), xonxoff, rtscts".to_string())
    }
}

fn possible_stop_bits(s: &str) -> Result<StopBits, String> {
    match s {
        "1" => Ok(StopBits::One),
        "2" => Ok(StopBits::Two),
        _ => Err("Possible values: 1 (default), 2".to_string())
    }
}

pub fn run() -> Result<(AppType, String, SerialConfig)> {
    let cli = Cli::parse();

    let device = cli.device.unwrap_or_default();

    let config = SerialConfig {
        baudrate: cli.baudrate.unwrap_or(115200),
        char_size: cli.char_size.unwrap_or(CharSize::Bits8),
        parity: cli.parity.unwrap_or(Parity::None),
        flow_control: cli.flow_control.unwrap_or(FlowControl::None),
        stop_bits: cli.stop_bits.unwrap_or(StopBits::One),
    };

    if cli.no_gui {
        return Ok((AppType::Tui, device, config))
    }

    Ok((AppType::Gui, device, config))
}