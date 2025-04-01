use anyhow::Result;
use std::{fmt::Write, time::Duration};

pub fn format_hex(data: &[u8], hex_string: &mut String) -> Result<()> {
    for byte in data {
        write!(hex_string, "{:02X} ", byte)?;
    }
    Ok(())
}

pub fn calculate_repaint_duration(fps: u32) -> Duration {
    if fps == 0 {
        return Duration::from_secs(u64::MAX);
    }
    let milliseconds_per_frame = 1000 / fps;
    Duration::from_millis(milliseconds_per_frame as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_hex_empty() -> Result<()> {
        let data: &[u8] = &[];
        let mut hex_string = String::new();
        format_hex(data, &mut hex_string)?;
        assert_eq!(hex_string, "");
        Ok(())
    }

    #[test]
    fn test_format_hex_single_byte() -> Result<()> {
        let data: &[u8] = &[0x41];
        let mut hex_string = String::new();
        format_hex(data, &mut hex_string)?;
        assert_eq!(hex_string, "41 ");
        Ok(())
    }

    #[test]
    fn test_format_hex_multiple_bytes() -> Result<()> {
        let data: &[u8] = &[0x01, 0x0A, 0xFF, 0x10];
        let mut hex_string = String::new();
        format_hex(data, &mut hex_string)?;
        assert_eq!(hex_string, "01 0A FF 10 ");
        Ok(())
    }

    #[test]
    fn test_calculate_repaint_duration_zero_fps() {
        assert_eq!(calculate_repaint_duration(0), Duration::from_secs(u64::MAX));
    }

    #[test]
    fn test_calculate_repaint_duration_common_fps() {
        assert_eq!(calculate_repaint_duration(30), Duration::from_millis(33));
        assert_eq!(calculate_repaint_duration(60), Duration::from_millis(16));
        assert_eq!(calculate_repaint_duration(120), Duration::from_millis(8));
    }

    #[test]
    fn test_calculate_repaint_duration_uncommon_fps() {
        assert_eq!(calculate_repaint_duration(25), Duration::from_millis(40));
        assert_eq!(calculate_repaint_duration(144), Duration::from_millis(6));
    }
}