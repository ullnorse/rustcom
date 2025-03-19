use anyhow::Result;
use std::thread;
use std::time::Duration;
use std::{
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Clone)]
pub struct Macro {
    pub text: String,
    pub delay: u32,
    pub repeat: bool,
    pub active: Arc<AtomicBool>,
}

impl Default for Macro {
    fn default() -> Self {
        Self {
            text: String::new(),
            delay: 1000,
            repeat: false,
            active: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub struct Macros {
    _macros: [Macro; 16],
    _config_file: String,
}

impl Default for Macros {
    fn default() -> Self {
        Self {
            _macros: core::array::from_fn(|_| Macro::default()),
            _config_file: String::new(),
        }
    }
}

impl Macros {
    pub fn new() -> Self {
        Self::default()
    }

    fn _save_config_to_file(&mut self, path: &Path) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut config = String::new();
        for i in 0..self._macros.len() {
            config.push_str(&format!("M{}\n", i + 1));
            config.push_str(&self._macros[i].text);
            config.push('\n');
        }

        file.write_all(config.as_bytes())?;

        Ok(())
    }

    fn _read_config_from_file(&mut self, path: &Path) {
        if let Ok(config) = std::fs::read_to_string(path) {
            for (i, line) in config.lines().skip(1).step_by(2).enumerate() {
                if i >= self._macros.len() {
                    break;
                }

                self._macros[i].text = line.trim().to_string();
            }
        }
    }

    fn _handle_macro_repeat(&mut self, index: usize) {
        let mac = &mut self._macros[index];

        if mac.repeat && !mac.active.load(Ordering::SeqCst) {
            let delay = mac.delay;
            let active_flag = mac.active.clone();
            active_flag.store(true, Ordering::SeqCst);

            let _text = mac.text.clone();

            thread::spawn(move || {
                while active_flag.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(delay as u64));
                }
            });
        } else if !mac.repeat && mac.active.load(Ordering::SeqCst) {
            mac.active.store(false, Ordering::SeqCst);
        }
    }
}
