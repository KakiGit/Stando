use std::cell::Cell;

pub struct FunctionLogGuard {
    name: &'static str,
    outcome: Cell<&'static str>,
    enabled: bool,
}

impl FunctionLogGuard {
    pub fn new(name: &'static str) -> Self {
        let enabled = tracing::enabled!(tracing::Level::DEBUG);
        if enabled {
            tracing::debug!(function = name, event = "enter");
        }
        Self {
            name,
            outcome: Cell::new("success"),
            enabled,
        }
    }

    pub fn mark_error(&self) {
        self.outcome.set("error");
    }
}

impl Drop for FunctionLogGuard {
    fn drop(&mut self) {
        if self.enabled {
            tracing::debug!(
                function = self.name,
                event = "exit",
                outcome = self.outcome.get()
            );
        }
    }
}

pub fn function_guard(name: &'static str) -> FunctionLogGuard {
    FunctionLogGuard::new(name)
}
