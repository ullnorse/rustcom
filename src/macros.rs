use anyhow::Result;
use std::{
    array,
    io::Write,
    path::Path,
    sync::{Arc, atomic::AtomicBool},
    thread::JoinHandle,
};

#[derive(Clone, Debug)]
pub struct Macro {
    pub text: String,
    pub delay: u32,
    pub repeat: bool,
}

impl Default for Macro {
    fn default() -> Self {
        Self {
            text: String::new(),
            delay: 1000,
            repeat: false,
        }
    }
}

pub struct Macros {
    pub macros: [Macro; Macros::NUM_OF_MACROS],
    pub config_file: String,
    pub running_threads: [Option<JoinHandle<()>>; Macros::NUM_OF_MACROS],
    pub stop_signals: [Arc<AtomicBool>; Macros::NUM_OF_MACROS],
}

impl Default for Macros {
    fn default() -> Self {
        Self {
            macros: core::array::from_fn(|_| Macro::default()),
            config_file: String::new(),
            running_threads: array::from_fn(|_| Default::default()),
            stop_signals: array::from_fn(|_| Arc::new(AtomicBool::default())),
        }
    }
}

impl Macros {
    pub const NUM_OF_MACROS: usize = 16;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_macro(&mut self, num: usize) -> Option<&mut Macro> {
        self.macros.get_mut(num)
    }

    pub fn set_macro(&mut self, num: usize, new_macro: Macro) {
        if num < Self::NUM_OF_MACROS {
            self.macros[num] = new_macro;
        }
    }

    pub fn get_config_file(&self) -> &str {
        &self.config_file
    }

    pub fn set_config_file(&mut self, config_file: String) {
        self.config_file = config_file;
    }

    pub fn save_config_to_file(&mut self, path: &Path) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut config = String::new();
        for i in 0..Self::NUM_OF_MACROS {
            config.push_str(&format!("M{}\n", i + 1));
            config.push_str(&self.macros[i].text);
            config.push('\n');
        }

        file.write_all(config.as_bytes())?;

        Ok(())
    }

    pub fn read_config_from_file(&mut self, path: &Path) {
        let Ok(config) = std::fs::read_to_string(path) else {
            return;
        };

        for (macro_slot, line) in self
            .macros
            .iter_mut()
            .zip(config.lines().skip(1).step_by(2))
        {
            macro_slot.text = line.trim().to_string();
        }
    }
}
