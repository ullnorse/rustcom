use anyhow::Result;
use base::serial::SerialConfig;

pub fn run(_device: String, _config: SerialConfig) -> Result<()> {
    println!("Hello from tui");

    Ok(())
}