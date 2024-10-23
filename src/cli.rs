use anyhow::Result;
use clap::Parser;
use serialport5::{FlowControl, DataBits, Parity, StopBits};
use super::serial::SerialSettings;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Serial device name")]
    device: Option<String>,

    #[arg(short, long, value_parser = possible_baudrates, help = "Possible values: 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 1000000 | default = 115200")]
    baudrate: Option<u32>,

    #[arg(short = 't', long, value_parser = possible_data_bits, help = "Possible values: 5, 6, 7, 8                                                   | default = 8")]
    data_bits: Option<DataBits>,

    #[arg(short, long, value_parser = possible_parity, help = "Possible values: none, odd, even                                              | default = none")]
    parity: Option<Parity>,

    #[arg(short, long, value_parser = possible_flow_control, help = "Possible values: none, software, hardware                                     | default = none")]
    flow_control: Option<FlowControl>,

    #[arg(short, long, value_parser = possible_stop_bits, help = "Possible values: 1, 2                                                         | default = 1")]
    stop_bits: Option<StopBits>,
}

fn possible_baudrates(s: &str) -> Result<u32, String> {
    match s {
        "1200" => Ok(1200),
        "2400" => Ok(2400),
        "4800" => Ok(4800),
        "9600" => Ok(9600),
        "19200" => Ok(19200),
        "38400" => Ok(38400),
        "57600" => Ok(57600),
        "115200" => Ok(115200),
        "1000000" => Ok(1000000),
        _ => Err("Possible values: 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 1000000 | default = 115200".to_string())
    }
}

fn possible_data_bits(s: &str) -> Result<DataBits, String> {
    match s {
        "5" => Ok(DataBits::Five),
        "6" => Ok(DataBits::Six),
        "7" => Ok(DataBits::Seven),
        "8" => Ok(DataBits::Eight),
        _ => Err("Possible values: 5, 6, 7, 8 | default = 8".to_string())
    }
}

fn possible_parity(s: &str) -> Result<Parity, String> {
    match s {
        "none" => Ok(Parity::None),
        "even" => Ok(Parity::Even),
        "odd" => Ok(Parity::Odd),
        _ => Err("Possible values: none, odd, even | default = none".to_string())
    }
}

fn possible_flow_control(s: &str) -> Result<FlowControl, String> {
    match s {
        "none" => Ok(FlowControl::None),
        "software" => Ok(FlowControl::Software),
        "Hardware" => Ok(FlowControl::Hardware),
        _ => Err("Possible values: none, software, hardware | default = none".to_string())
    }
}

fn possible_stop_bits(s: &str) -> Result<StopBits, String> {
    match s {
        "1" => Ok(StopBits::One),
        "2" => Ok(StopBits::Two),
        _ => Err("Possible values: 1, 2 | default = 1".to_string())
    }
}

pub fn run() -> Result<(String, SerialSettings)> {
    let cli = Cli::parse();

    let device = cli.device.unwrap_or_default();

    let settings = SerialSettings {
        baud_rate: cli.baudrate.unwrap_or(115200),
        data_bits: cli.data_bits.unwrap_or(DataBits::Eight),
        parity: cli.parity.unwrap_or(Parity::None),
        flow_control: cli.flow_control.unwrap_or(FlowControl::None),
        stop_bits: cli.stop_bits.unwrap_or(StopBits::One),
    };

    Ok((device, settings))
}