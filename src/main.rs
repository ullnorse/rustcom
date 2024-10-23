use rustcom::cli;
use rustcom::app;
use anyhow::Result;

fn main() -> Result<()> {
    let (device, settings) = cli::run()?;
    app::run(device, settings)
}
