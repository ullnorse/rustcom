use crate::serial::{DataBits, FlowControl, Parity, SerialSettings, StopBits};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Serial device name")]
    device: Option<String>,

    #[arg(short, long, value_parser = possible_baudrates, help = "Possible values: 0, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 1000000 | default = 115200")]
    baudrate: Option<u32>,

    #[arg(short = 't', long, value_parser = possible_data_bits, help = "Possible values: 5, 6, 7, 8                                                      | default = 8")]
    data_bits: Option<DataBits>,

    #[arg(short, long, value_parser = possible_parity, help = "Possible values: none, odd, even                                                 | default = none")]
    parity: Option<Parity>,

    #[arg(short, long, value_parser = possible_flow_control, help = "Possible values: none, software, hardware                                        | default = none")]
    flow_control: Option<FlowControl>,

    #[arg(short, long, value_parser = possible_stop_bits, help = "Possible values: 1, 2                                                            | default = 1")]
    stop_bits: Option<StopBits>,
}

fn possible_baudrates(s: &str) -> Result<u32, String> {
    match s {
        "0" => Ok(0),
        "1200" => Ok(1200),
        "2400" => Ok(2400),
        "4800" => Ok(4800),
        "9600" => Ok(9600),
        "19200" => Ok(19200),
        "38400" => Ok(38400),
        "57600" => Ok(57600),
        "115200" => Ok(115200),
        "1000000" => Ok(1000000),
        _ => Err("Possible values: 0, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 1000000 | default = 115200".to_string()),
    }
}

fn possible_data_bits(s: &str) -> Result<DataBits, String> {
    match s {
        "5" => Ok(DataBits::Five),
        "6" => Ok(DataBits::Six),
        "7" => Ok(DataBits::Seven),
        "8" => Ok(DataBits::Eight),
        _ => Err("Possible values: 5, 6, 7, 8 | default = 8".to_string()),
    }
}

fn possible_parity(s: &str) -> Result<Parity, String> {
    match s {
        "none" => Ok(Parity::None),
        "even" => Ok(Parity::Even),
        "odd" => Ok(Parity::Odd),
        _ => Err("Possible values: none, odd, even | default = none".to_string()),
    }
}

fn possible_flow_control(s: &str) -> Result<FlowControl, String> {
    match s {
        "none" => Ok(FlowControl::None),
        "software" => Ok(FlowControl::Software),
        "hardware" => Ok(FlowControl::Hardware),
        _ => Err("Possible values: none, software, hardware | default = none".to_string()),
    }
}

fn possible_stop_bits(s: &str) -> Result<StopBits, String> {
    match s {
        "1" => Ok(StopBits::One),
        "2" => Ok(StopBits::Two),
        _ => Err("Possible values: 1, 2 | default = 1".to_string()),
    }
}

pub fn run() -> SerialSettings {
    let cli = Cli::parse();

    SerialSettings {
        port: cli.device.unwrap_or_default(),
        baud_rate: cli.baudrate.unwrap_or(115200),
        data_bits: cli.data_bits.unwrap_or(DataBits::Eight),
        parity: cli.parity.unwrap_or(Parity::None),
        flow_control: cli.flow_control.unwrap_or(FlowControl::None),
        stop_bits: cli.stop_bits.unwrap_or(StopBits::One),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_baudrates() {
        assert_eq!(possible_baudrates("115200"), Ok(115200));
        assert_eq!(possible_baudrates("9600"), Ok(9600));
        assert_eq!(possible_baudrates("0"), Ok(0));
    }

    #[test]
    fn test_invalid_baudrate() {
        assert!(possible_baudrates("12345").is_err());
        assert!(possible_baudrates("invalid").is_err());
    }

    #[test]
    fn test_valid_data_bits() {
        assert_eq!(possible_data_bits("8"), Ok(DataBits::Eight));
        assert_eq!(possible_data_bits("5"), Ok(DataBits::Five));
    }

    #[test]
    fn test_invalid_data_bits() {
        assert!(possible_data_bits("9").is_err());
        assert!(possible_data_bits("abc").is_err());
    }

    #[test]
    fn test_valid_parity() {
        assert_eq!(possible_parity("none"), Ok(Parity::None));
        assert_eq!(possible_parity("even"), Ok(Parity::Even));
        assert_eq!(possible_parity("odd"), Ok(Parity::Odd));
    }

    #[test]
    fn test_invalid_parity() {
        assert!(possible_parity("invalid").is_err());
    }

    #[test]
    fn test_valid_flow_control() {
        assert_eq!(possible_flow_control("none"), Ok(FlowControl::None));
        assert_eq!(possible_flow_control("software"), Ok(FlowControl::Software));
        assert_eq!(possible_flow_control("hardware"), Ok(FlowControl::Hardware));
    }

    #[test]
    fn test_valid_stop_bits() {
        assert_eq!(possible_stop_bits("1"), Ok(StopBits::One));
        assert_eq!(possible_stop_bits("2"), Ok(StopBits::Two));
    }

    #[test]
    fn test_invalid_stop_bits() {
        assert!(possible_stop_bits("3").is_err());
    }
}
