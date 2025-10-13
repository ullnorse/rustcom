use crate::app::App;
use anyhow::{Result, anyhow};
use clipboard::ClipboardProvider;

impl App {
    pub fn cut(&mut self) -> Result<()> {
        self.copy()?;
        self.output_text.clear();
        Ok(())
    }

    pub fn copy(&mut self) -> Result<()> {
        self.clipboard
            .set_contents(self.output_text.clone())
            .map_err(|e| anyhow!("Failed to copy to clipboard: {e}"))
    }

    pub fn paste(&mut self) -> Result<()> {
        let contents = self
            .clipboard
            .get_contents()
            .map_err(|e| anyhow!("Failed to paste from clipboard: {e}"))?;
        self.input_text.push_str(&contents);
        Ok(())
    }
}
