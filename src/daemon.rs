use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Daemon {
    running: Arc<AtomicBool>,
}

impl Daemon {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn run<F>(&self, mut main_loop: F) -> Result<()>
    where
        F: FnMut() -> Result<()>,
    {
        while self.is_running() {
            main_loop()?;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(())
    }
}

// Systemd integration can be added later as an optional feature
// For now, this provides basic daemon functionality
