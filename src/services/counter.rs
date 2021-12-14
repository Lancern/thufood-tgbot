use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// Provide a basic counter service.
pub struct CounterService {
    path: PathBuf,
    counter: AtomicU64,
    write_file_lock: Mutex<()>,
}

impl CounterService {
    /// Create a new counter service with the given file as the backing file.
    pub fn new<P>(file_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path = file_path.into();
        let init_counter: u64 = {
            match std::fs::read_to_string(&path) {
                Ok(s) => match s.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::warn!("Failed to parse meow counter: {}", e);
                        0
                    }
                },
                Err(e) => {
                    log::warn!("Failed to read meow counter from file: {}", e);
                    0
                }
            }
        };

        Self {
            path,
            counter: AtomicU64::new(init_counter),
            write_file_lock: Mutex::new(()),
        }
    }

    /// Increase the counter and get the updated counter value.
    pub fn increase(&self) -> u64 {
        let ret = self.counter.fetch_add(1, Ordering::Relaxed) + 1;

        let _lock = self.write_file_lock.lock().unwrap();
        if let Err(e) = std::fs::write(&self.path, ret.to_string()) {
            log::warn!("Failed to write counter into file: {}", e);
        }

        ret
    }
}
