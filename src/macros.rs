use anyhow::Result;
use std::{
    array,
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
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
    macros: [Macro; Macros::NUM_OF_MACROS],
    config_file: String,
    running_threads: [Option<JoinHandle<()>>; Macros::NUM_OF_MACROS],
    stop_signals: [Arc<AtomicBool>; Macros::NUM_OF_MACROS],
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

    pub fn start_macro(&mut self, num: usize) {
        let Some(m) = self.macros.get(num) else {
            return;
        };

        let Some(s) = self.stop_signals.get(num) else {
            return;
        };

        let stop_signal = s.clone();
        let macro_text = m.text.clone();
        let delay = m.delay;

        stop_signal.store(true, Ordering::SeqCst);

        let handle = thread::spawn(move || {
            while stop_signal.load(Ordering::SeqCst) {
                println!("Sending macro text: {macro_text}");
                thread::sleep(Duration::from_millis(delay as u64));
            }
        });

        self.running_threads[num] = Some(handle);
    }

    pub fn stop_macro(&mut self, num: usize) {
        let Some(stop_signal) = self.stop_signals.get(num) else {
            return;
        };

        let Some(handle) = self.running_threads.get_mut(num) else {
            return;
        };

        stop_signal.store(false, Ordering::SeqCst);
        handle.take();
    }
}
