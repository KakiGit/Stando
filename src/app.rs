use crate::ai::AIService;
use crate::config::Config;
use crate::daemon::Daemon;
use crate::hotkeys::{HotKeyEvent, HotkeyManager};
use crate::logging;
use crate::search::SearchEngine;
use crate::tray::TrayIcon;
use crate::ui::SearchWindow;
use adw::Application;
use anyhow::{Context, Result};
use futures::channel::oneshot;
use gdk4::Key;
use gtk4::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use tokio::sync::RwLock;

pub struct App {
    #[allow(dead_code)]
    application: Application,
    config: Config,
    search_engine: Arc<SearchEngine>,
    ai_service: Option<Arc<AIService>>,
    search_window: Rc<SearchWindow>,
    hotkey_manager: Option<Arc<HotkeyManager>>,
    #[allow(dead_code)]
    daemon: Option<Arc<Daemon>>,
    ai_mode: Arc<RwLock<bool>>,
    #[allow(dead_code)]
    tray_icon: Option<Arc<TrayIcon>>,
}

impl App {
    pub fn new(application: Application) -> Result<Self> {
        let log_guard = logging::function_guard("App::new");
        let result = (|| {
            // Load config
            let config = Config::load().context("Failed to load configuration")?;

            // Initialize search engine
            let search_engine = Arc::new(SearchEngine::new());

            // Initialize AI service if API key is available
            let ai_service = config
                .openai_api_key
                .as_ref()
                .map(|key| Arc::new(AIService::new(key.clone(), search_engine.clone())));

            // Create search window
            let search_window = Rc::new(
                SearchWindow::new(&application, &config.hotkey_ai_toggle)
                    .context("Failed to create search window")?,
            );

            // Initialize hotkey manager
            let hotkey_manager = HotkeyManager::new(&config).ok().map(Arc::new);

            // Initialize tray icon
            let tray_icon = TrayIcon::new(&application).ok().map(Arc::new);
            if let Some(ref tray) = tray_icon {
                let search_window_clone = search_window.clone();
                tray.connect_show(move || {
                    search_window_clone.show();
                });
                tray.show();
            }

            let ai_mode = Arc::new(RwLock::new(false));

            Ok(Self {
                application,
                config,
                search_engine,
                ai_service,
                search_window,
                hotkey_manager,
                daemon: None,
                ai_mode,
                tray_icon,
            })
        })();
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub async fn initialize(&self) -> Result<()> {
        let log_guard = logging::function_guard("App::initialize");
        let result = async {
            // Index files and applications
            self.search_engine
                .index_files(&self.config.search_paths)
                .await
                .context("Failed to index files")?;

            self.search_engine
                .index_applications()
                .await
                .context("Failed to index applications")?;

            // Set up UI callbacks
            self.setup_ui_callbacks()?;

            // Set up window-local shortcuts
            self.setup_window_shortcuts()?;

            // Set up hotkey polling
            self.setup_hotkey_polling();

            // Initialize AI button state
            self.update_ai_button_state();

            // Show the main window on start
            self.show_window();

            Ok(())
        }
        .await;
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    fn setup_ui_callbacks(&self) -> Result<()> {
        let log_guard = logging::function_guard("App::setup_ui_callbacks");
        let result = {
            let search_engine = self.search_engine.clone();
            let config = self.config.clone();
            let ai_service = self.ai_service.clone();
            let ai_mode = self.ai_mode.clone();

            // Search on entry change
            let entry = self.search_window.entry().clone();
            let search_window_for_entry = self.search_window.clone();
            let ai_mode_for_entry = ai_mode.clone();
            entry.connect_changed(move |entry| {
                let query = entry.text().to_string();
                let search_window = search_window_for_entry.clone();
                let search_engine = search_engine.clone();
                let max_results = config.max_results;
                let ai_mode = ai_mode_for_entry.clone();

                // Perform search asynchronously
                glib::MainContext::default().spawn_local(async move {
                    let is_ai_mode = *ai_mode.read().await;
                    if query.is_empty() {
                        search_window.clear_results().await;
                        return;
                    }

                    if is_ai_mode {
                        if let Some(at_index) = query.rfind('@') {
                            let search_query = query[at_index + 1..].trim();
                            if search_query.is_empty() {
                                search_window.clear_results().await;
                                return;
                            }
                            let results = search_engine.search(search_query, max_results).await;
                            search_window.update_results(results).await;
                        } else {
                            search_window.clear_results().await;
                        }
                        return;
                    }

                    let results = search_engine.search(&query, max_results).await;
                    search_window.update_results(results).await;
                });
            });

            // Handle entry activate (Enter key)
            let search_window_activate = self.search_window.clone();
            let ai_mode_clone = ai_mode.clone();
            let ai_service_clone = ai_service.clone();
            self.search_window.connect_entry_activate(move || {
            let search_window_clone = search_window_activate.clone();
            let ai_mode = ai_mode_clone.clone();
            let ai_service = ai_service_clone.clone();
            glib::MainContext::default().spawn_local(async move {
                // Check if AI mode is enabled
                let is_ai_mode = *ai_mode.read().await;

                if is_ai_mode {
                    // In AI mode: send query to AI
                    let query = search_window_clone.get_query();
                    if query.is_empty() {
                        return;
                    }

                    if let Some(ai_service) = ai_service {
                        // Show a lightweight loading row while waiting for AI
                        let loading_result = crate::search::SearchResult::Text {
                            content: "Thinking...".to_string(),
                            name: "AI".to_string(),
                        };
                        search_window_clone.update_results(vec![loading_result]).await;
                        let (tx, rx) = oneshot::channel();
                        let ai_service = ai_service.clone();
                        let query_for_thread = query.clone();
                        thread::spawn(move || {
                            let result = ai_service.process_query_blocking(&query_for_thread);
                            let _ = tx.send(result);
                        });

                        match rx.await {
                            Ok(Ok(response)) => {
                                // Display AI response as a text result
                                let ai_result = crate::search::SearchResult::Text {
                                    content: response.clone(),
                                    name: format!("AI Response: {}", query),
                                };
                                search_window_clone.update_results(vec![ai_result]).await;
                            }
                            Ok(Err(e)) => {
                                eprintln!("AI query failed: {}", e);
                                // Show error as text result
                                let error_result = crate::search::SearchResult::Text {
                                    content: format!("Error: {}", e),
                                    name: "AI Error".to_string(),
                                };
                                search_window_clone.update_results(vec![error_result]).await;
                            }
                            Err(_) => {
                                let error_result = crate::search::SearchResult::Text {
                                    content: "Error: AI request thread terminated.".to_string(),
                                    name: "AI Error".to_string(),
                                };
                                search_window_clone.update_results(vec![error_result]).await;
                            }
                        }
                    } else {
                        // AI service not available
                        let error_result = crate::search::SearchResult::Text {
                            content: "AI service not configured. Please set OPENAI_API_KEY in your config.".to_string(),
                            name: "AI Not Available".to_string(),
                        };
                        search_window_clone.update_results(vec![error_result]).await;
                    }
                } else {
                    // Normal mode: open selected result
                    if let Some(result) = search_window_clone.get_selected_result().await {
                        Self::open_result(&result);
                        search_window_clone.hide();
                    }
                }
            });
        });

            // Handle list activation
            let search_window_list = self.search_window.clone();
            self.search_window.connect_activate(move || {
                let search_window_clone = search_window_list.clone();
                glib::MainContext::default().spawn_local(async move {
                    if let Some(result) = search_window_clone.get_selected_result().await {
                        Self::open_result(&result);
                        search_window_clone.hide();
                    }
                });
            });

            // Handle Escape key
            let search_window_key = self.search_window.clone();
            self.search_window
                .connect_key_press(move |keyval| match keyval {
                    key if key == Key::Escape => {
                        search_window_key.hide();
                        true
                    }
                    key if key == Key::Up || key == Key::KP_Up => {
                        search_window_key.move_selection_up();
                        true
                    }
                    key if key == Key::Down || key == Key::KP_Down => {
                        search_window_key.move_selection_down();
                        true
                    }
                    key if key == Key::Tab || key == Key::ISO_Left_Tab => {
                        search_window_key.autocomplete_selected()
                    }
                    _ => false,
                });

            // Handle AI button click
            let ai_mode_clone = ai_mode.clone();
            let search_window_ai = self.search_window.clone();
            self.search_window.connect_ai_button_clicked(move || {
                Self::spawn_toggle_ai(ai_mode_clone.clone(), search_window_ai.clone());
            });

            Ok(())
        };
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub fn setup_hotkey_polling(&self) {
        let _log_guard = logging::function_guard("App::setup_hotkey_polling");
        if let Some(hotkey_manager) = &self.hotkey_manager {
            let search_window = self.search_window.clone();
            let hotkey_manager = hotkey_manager.clone();

            // Poll for hotkey events
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if let Ok(Some(event)) = hotkey_manager.try_recv() {
                    match event {
                        HotKeyEvent::ShowHide => {
                            println!("Show Hide");
                            if search_window.is_visible() {
                                search_window.hide();
                            } else {
                                search_window.show();
                            }
                        }
                    }
                }
                glib::ControlFlow::Continue
            });
        }
    }

    fn setup_window_shortcuts(&self) -> Result<()> {
        let log_guard = logging::function_guard("App::setup_window_shortcuts");
        let result = {
            let window = self.search_window.window();
            let action = gio::SimpleAction::new("toggle-ai-mode", None);
            let ai_mode = self.ai_mode.clone();
            let search_window = self.search_window.clone();
            action.connect_activate(move |_, _| {
                Self::spawn_toggle_ai(ai_mode.clone(), search_window.clone());
            });
            window.add_action(&action);

            let controller = gtk4::ShortcutController::new();
            controller.set_scope(gtk4::ShortcutScope::Local);
            if let Some(trigger) =
                gtk4::ShortcutTrigger::parse_string(&self.config.hotkey_ai_toggle)
            {
                let action = gtk4::NamedAction::new("win.toggle-ai-mode");
                let shortcut = gtk4::Shortcut::new(Some(trigger), Some(action));
                controller.add_shortcut(shortcut);
            } else {
                log_guard.mark_error();
            }
            window.add_controller(controller);
            Ok(())
        };
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    fn spawn_toggle_ai(ai_mode: Arc<RwLock<bool>>, search_window: Rc<SearchWindow>) {
        glib::MainContext::default().spawn_local(async move {
            Self::toggle_ai_mode(ai_mode, search_window).await;
        });
    }

    async fn toggle_ai_mode(ai_mode: Arc<RwLock<bool>>, search_window: Rc<SearchWindow>) {
        let mut mode = ai_mode.write().await;
        *mode = !*mode;
        let enabled = *mode;
        drop(mode);
        // Update UI to show AI mode status
        search_window.set_ai_mode(enabled);
        if enabled {
            search_window.clear_results().await;
        }
    }

    pub fn show_window(&self) {
        let _log_guard = logging::function_guard("App::show_window");
        self.search_window.show();
    }

    #[allow(dead_code)]
    pub fn hide_window(&self) {
        let _log_guard = logging::function_guard("App::hide_window");
        self.search_window.hide();
    }

    fn update_ai_button_state(&self) {
        let _log_guard = logging::function_guard("App::update_ai_button_state");
        let ai_mode = self.ai_mode.clone();
        let search_window = self.search_window.clone();
        glib::MainContext::default().spawn_local(async move {
            let enabled = *ai_mode.read().await;
            search_window.set_ai_mode(enabled);
        });
    }

    #[allow(dead_code)]
    pub fn toggle_window(&self) {
        let _log_guard = logging::function_guard("App::toggle_window");
        if self.search_window.is_visible() {
            self.hide_window();
        } else {
            self.show_window();
        }
    }

    #[allow(dead_code)]
    pub fn set_daemon_mode(&mut self, enabled: bool) {
        let _log_guard = logging::function_guard("App::set_daemon_mode");
        if enabled {
            self.daemon = Some(Arc::new(Daemon::new()));
        } else {
            self.daemon = None;
        }
    }

    fn open_result(result: &crate::search::SearchResult) {
        let _log_guard = logging::function_guard("App::open_result");
        match result {
            crate::search::SearchResult::File { path, .. } => {
                // Open file with default application
                if let Err(e) = open::that(path) {
                    eprintln!("Failed to open file: {}", e);
                }
            }
            crate::search::SearchResult::Application { exec, .. } => {
                // Execute application
                // Parse exec command (simplified - desktop files can have %u, %f, etc.)
                // Remove desktop file field codes
                let exec_clean = exec
                    .replace("%u", "")
                    .replace("%U", "")
                    .replace("%f", "")
                    .replace("%F", "")
                    .trim()
                    .to_string();

                let parts: Vec<&str> = exec_clean.split_whitespace().collect();
                if let Some(command) = parts.first() {
                    let mut cmd = std::process::Command::new(command);
                    if parts.len() > 1 {
                        cmd.args(&parts[1..]);
                    }
                    if let Err(e) = cmd.spawn() {
                        eprintln!("Failed to launch application: {}", e);
                    }
                }
            }
            crate::search::SearchResult::Text { .. } => {
                // Text results (like AI responses) don't need to be opened
                // They're just displayed in the list
            }
        }
    }
}
