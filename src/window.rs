use crate::config::FloatingPreference;
use adw::Application;

/// Tracks the runtime stacking data used during floating-mode operations.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StandoWindowState {
    pub is_floating: bool,
    pub workspace_id: Option<String>,
    pub display_id: Option<String>,
    pub last_override_success: bool,
}

impl Default for StandoWindowState {
    fn default() -> Self {
        Self {
            is_floating: false,
            workspace_id: None,
            display_id: None,
            last_override_success: true,
        }
    }
}

/// Coordinates GTK stacking hints and floating-mode state transitions.
#[allow(dead_code)]
pub struct FloatingWindowController {
    application: Application,
    state: StandoWindowState,
    preference: FloatingPreference,
}

#[allow(dead_code)]
impl FloatingWindowController {
    /// Creates a new controller tied to the provided GTK application.
    pub fn new(application: &Application, preference: FloatingPreference) -> Self {
        Self {
            application: application.clone(),
            state: StandoWindowState::default(),
            preference,
        }
    }

    /// Placeholder helper to initialize any controller resources.
    pub fn initialize(&self) {
        // TODO: Wire GTK/GLib observers once workspace/display signals are available.
    }

    /// Placeholder for reasserting floating hints without doing anything yet.
    pub fn reassert_floating(&mut self) -> bool {
        // TODO: Reapply stacking hints in response to workspace/display updates.
        self.state.is_floating
    }

    /// Placeholder toggle used by the UI in later phases.
    pub fn set_floating(&mut self, enabled: bool) {
        self.state.is_floating = enabled;
        // TODO: Persist preference and update GTK stacking hints.
    }

    /// Returns a snapshot of the current window state.
    pub fn state(&self) -> &StandoWindowState {
        &self.state
    }

    /// Returns the stored user preference for floating mode.
    pub fn preference(&self) -> &FloatingPreference {
        &self.preference
    }
}
