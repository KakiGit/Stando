use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::logging;

pub struct Daemon {
    running: Arc<AtomicBool>,
}

impl Daemon {
    pub fn new() -> Self {
        let _log_guard = logging::function_guard("Daemon::new");
        Self {
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn is_running(&self) -> bool {
        let _log_guard = logging::function_guard("Daemon::is_running");
        self.running.load(Ordering::Relaxed)
    }

    pub fn stop(&self) {
        let _log_guard = logging::function_guard("Daemon::stop");
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn run<F>(&self, mut main_loop: F) -> Result<()>
    where
        F: FnMut() -> Result<()>,
    {
        let log_guard = logging::function_guard("Daemon::run");
        let result = (|| {
            while self.is_running() {
                main_loop()?;
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Ok(())
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }
}

// Systemd integration can be added later as an optional feature
// For now, this provides basic daemon functionality
