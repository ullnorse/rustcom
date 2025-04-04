use crate::app::App;
use anyhow::{Result, bail};
use clipboard::{ClipboardContext, ClipboardProvider};

impl App {
    pub fn cut(&mut self) -> Result<()> {
        self.copy()?;
        self.output_text.clear();
        Ok(())
    }

    pub fn copy(&mut self) -> Result<()> {
        set_clipboard_contents(&mut self.clipboard, self.output_text.clone())
    }

    pub fn paste(&mut self) -> Result<()> {
        get_clipboard_contents(&mut self.clipboard)
            .map(|contents| self.input_text.push_str(&contents))
    }
}

fn set_clipboard_contents(clipboard: &mut ClipboardContext, contents: String) -> Result<()> {
    match clipboard.set_contents(contents) {
        Ok(_) => Ok(()),
        Err(e) => bail!("Error setting clipboard contents: {e:?}"),
    }
}

fn get_clipboard_contents(clipboard: &mut ClipboardContext) -> Result<String> {
    match clipboard.get_contents() {
        Ok(contents) => Ok(contents),
        Err(e) => bail!("Error getting clipboard contents: {e:?}"),
    }
}
