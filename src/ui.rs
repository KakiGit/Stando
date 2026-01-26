use crate::logging;
use crate::search::SearchResult;
use adw::Application;
use anyhow::{Context, Result};
use gdk4::prelude::*;
use gdk4::{Display, Monitor, Rectangle};
use gio::prelude::ListModelExt;
use glib::prelude::Cast;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box, Button, Entry, ListBox, ScrolledWindow};
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use xdg::BaseDirectories;

const DEFAULT_PLACEHOLDER: &str = "Search files and applications...";
const AI_PLACEHOLDER: &str = "What would you like to ask to AI?";
const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;

pub struct SearchWindow {
    window: ApplicationWindow,
    entry: Entry,
    ai_button: Button,
    ai_shortcut: String,
    list_box: ListBox,
    results: Arc<RwLock<Vec<SearchResult>>>,
    selected_index: Arc<RwLock<usize>>,
}

impl SearchWindow {
    pub fn new(app: &Application, ai_shortcut: &str) -> Result<Self> {
        let log_guard = logging::function_guard("SearchWindow::new");
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Stando")
            .resizable(true)
            .decorated(false)
            .build();
        window.add_css_class("search-window");

        // Center and size the window before showing it.
        Self::configure_window_geometry(&window);

        // Main container
        let main_box = Box::new(gtk4::Orientation::Vertical, 0);
        main_box.add_css_class("search-main");
        window.set_child(Some(&main_box));

        // Search bar container (horizontal box for entry + button)
        let search_box = Box::new(gtk4::Orientation::Horizontal, 8);
        search_box.add_css_class("search-bar");
        search_box.set_margin_start(16);
        search_box.set_margin_end(16);
        search_box.set_margin_top(16);
        search_box.set_margin_bottom(8);
        main_box.append(&search_box);

        // Search entry
        let entry = Entry::new();
        entry.set_placeholder_text(Some(DEFAULT_PLACEHOLDER));
        entry.set_hexpand(true);
        entry.set_margin_start(0);
        entry.set_margin_end(0);
        entry.set_css_classes(&["search-entry"]);
        search_box.append(&entry);

        // AI mode toggle button
        let ai_button = Button::new();
        ai_button.set_css_classes(&["ai-mode-button", "search-ai-button"]);
        ai_button.set_tooltip_text(Some(&format!("Toggle AI Mode ({})", ai_shortcut)));
        ai_button.set_label("AI");
        ai_button.set_valign(gtk4::Align::Center);
        search_box.append(&ai_button);

        // Results list
        let scrolled = ScrolledWindow::new();
        scrolled.set_css_classes(&["search-results-scroll"]);
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);

        let list_box = ListBox::new();
        list_box.set_css_classes(&["search-results-list"]);
        list_box.set_selection_mode(gtk4::SelectionMode::Single);
        scrolled.set_child(Some(&list_box));
        main_box.append(&scrolled);

        // Load CSS
        let provider = gtk4::CssProvider::new();
        let css_data = Self::load_css_data();
        provider.load_from_data(&css_data);
        if let Some(display) = gdk4::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Initially hidden
        // window.set_visible(false);

        let results = Arc::new(RwLock::new(Vec::new()));
        let selected_index = Arc::new(RwLock::new(0));

        let result = Ok(Self {
            window,
            entry,
            ai_button,
            ai_shortcut: ai_shortcut.to_string(),
            list_box,
            results,
            selected_index,
        });
        if result.is_err() {
            log_guard.mark_error();
        }
        result
    }

    pub fn window(&self) -> &ApplicationWindow {
        let _log_guard = logging::function_guard("SearchWindow::window");
        &self.window
    }

    pub fn entry(&self) -> &Entry {
        let _log_guard = logging::function_guard("SearchWindow::entry");
        &self.entry
    }

    #[allow(dead_code)]
    pub fn ai_button(&self) -> &Button {
        let _log_guard = logging::function_guard("SearchWindow::ai_button");
        &self.ai_button
    }

    pub fn set_ai_mode(&self, enabled: bool) {
        let _log_guard = logging::function_guard("SearchWindow::set_ai_mode");
        if enabled {
            self.ai_button.add_css_class("ai-mode-active");
            self.ai_button.set_tooltip_text(Some(&format!(
                "AI Mode: ON (Click to disable, Shortcut: {})",
                self.ai_shortcut
            )));
            self.entry.set_placeholder_text(Some(AI_PLACEHOLDER));
        } else {
            self.ai_button.remove_css_class("ai-mode-active");
            self.ai_button.set_tooltip_text(Some(&format!(
                "AI Mode: OFF (Click to enable, Shortcut: {})",
                self.ai_shortcut
            )));
            self.entry.set_placeholder_text(Some(DEFAULT_PLACEHOLDER));
        }
    }

    fn load_css_data() -> String {
        match Self::load_css_from_config() {
            Ok(css) => css,
            Err(err) => {
                tracing::error!(
                    "Unable to load style.css from config directory, falling back to built-in styles: {err}"
                );
                include_str!("../assets/style.css").to_string()
            }
        }
    }

    fn load_css_from_config() -> Result<String> {
        let xdg_dirs = BaseDirectories::with_prefix("stando")
            .context("Failed to initialize XDG directories")?;
        let style_path = xdg_dirs
            .place_config_file("style.css")
            .context("Failed to determine style.css path")?;

        if let Some(parent) = style_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create directories for style.css configuration")?;
        }

        if !style_path.exists() {
            fs::write(&style_path, include_str!("../assets/style.css"))
                .context("Failed to write default style.css to config directory")?;
        }

        fs::read_to_string(&style_path).context("Failed to read config style.css")
    }

    pub fn connect_ai_button_clicked<F: Fn() + 'static>(&self, callback: F) {
        let _log_guard = logging::function_guard("SearchWindow::connect_ai_button_clicked");
        self.ai_button.connect_clicked(move |_| {
            callback();
        });
    }

    pub fn show(&self) {
        let _log_guard = logging::function_guard("SearchWindow::show");
        self.window.set_visible(true);
        self.window.present();
        self.entry.grab_focus();
    }

    pub fn hide(&self) {
        let _log_guard = logging::function_guard("SearchWindow::hide");
        self.window.set_visible(false);
    }

    pub fn is_visible(&self) -> bool {
        let _log_guard = logging::function_guard("SearchWindow::is_visible");
        self.window.is_visible()
    }

    pub async fn update_results(&self, new_results: Vec<SearchResult>) {
        let _log_guard = logging::function_guard("SearchWindow::update_results");
        *self.results.write().await = new_results.clone();

        // Clear existing rows
        while let Some(row) = self.list_box.row_at_index(0) {
            self.list_box.remove(&row);
        }

        // Add new results
        for result in new_results.iter() {
            let row = gtk4::ListBoxRow::new();
            row.set_css_classes(&["search-result-row"]);
            let display_text = match result {
                crate::search::SearchResult::Text { content, name } => {
                    // For text results, show full content and let the label wrap.
                    format!("{}\n{}", name, content)
                }
                _ => result.display_name().to_string(),
            };
            let label = gtk4::Label::new(Some(&display_text));
            label.set_halign(gtk4::Align::Start);
            label.set_xalign(0.0);
            label.set_hexpand(true);
            label.set_margin_start(10);
            label.set_margin_end(10);
            label.set_margin_top(5);
            label.set_margin_bottom(5);
            label.set_wrap(true);
            label.set_wrap_mode(gtk4::pango::WrapMode::Word);
            label.set_ellipsize(gtk4::pango::EllipsizeMode::None);
            label.set_css_classes(&["search-result-label"]);
            row.set_child(Some(&label));
            row.set_selectable(true);
            self.list_box.append(&row);
        }

        // Select first item
        if let Some(first_row) = self.list_box.row_at_index(0) {
            self.list_box.select_row(Some(&first_row));
            *self.selected_index.write().await = 0;
        }
    }

    pub async fn clear_results(&self) {
        let _log_guard = logging::function_guard("SearchWindow::clear_results");
        self.update_results(Vec::new()).await;
    }

    pub async fn get_selected_result(&self) -> Option<SearchResult> {
        let _log_guard = logging::function_guard("SearchWindow::get_selected_result");
        let results = self.results.read().await;
        let index = self.selected_index.read().await;
        results.get(*index).cloned()
    }

    pub fn connect_activate<F: Fn() + 'static>(&self, callback: F) {
        let _log_guard = logging::function_guard("SearchWindow::connect_activate");
        self.list_box.connect_row_activated(move |_, _| {
            callback();
        });
    }

    pub fn connect_entry_activate<F: Fn() + 'static>(&self, callback: F) {
        let _log_guard = logging::function_guard("SearchWindow::connect_entry_activate");
        self.entry.connect_activate(move |_| {
            callback();
        });
    }

    pub fn connect_key_press<F: Fn(gdk4::Key) -> bool + 'static>(&self, callback: F) {
        let _log_guard = logging::function_guard("SearchWindow::connect_key_press");
        let controller = gtk4::EventControllerKey::new();
        controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
            let handled = match keyval {
                gdk4::Key::Escape
                | gdk4::Key::Up
                | gdk4::Key::Down
                | gdk4::Key::KP_Up
                | gdk4::Key::KP_Down
                | gdk4::Key::Tab
                | gdk4::Key::ISO_Left_Tab => callback(keyval),
                _ => false,
            };

            if handled {
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        self.window.add_controller(controller);
    }

    pub fn get_query(&self) -> String {
        let _log_guard = logging::function_guard("SearchWindow::get_query");
        self.entry.text().to_string()
    }

    #[allow(dead_code)]
    pub fn set_query(&self, query: &str) {
        let _log_guard = logging::function_guard("SearchWindow::set_query");
        self.entry.set_text(query);
    }

    #[allow(dead_code)]
    pub fn clear_query(&self) {
        let _log_guard = logging::function_guard("SearchWindow::clear_query");
        self.entry.set_text("");
    }

    pub fn move_selection_down(&self) {
        let _log_guard = logging::function_guard("SearchWindow::move_selection_down");
        // Move selection down in list
        let current = *self.selected_index.blocking_read();
        let results_len = self.results.blocking_read().len();
        if current < results_len.saturating_sub(1) {
            *self.selected_index.blocking_write() = current + 1;
            if let Some(row) = self.list_box.row_at_index((current + 1) as i32) {
                self.list_box.select_row(Some(&row));
            }
        }
    }

    pub fn move_selection_up(&self) {
        let _log_guard = logging::function_guard("SearchWindow::move_selection_up");
        // Move selection up in list
        let current = *self.selected_index.blocking_read();
        if current > 0 {
            *self.selected_index.blocking_write() = current - 1;
            if let Some(row) = self.list_box.row_at_index((current - 1) as i32) {
                self.list_box.select_row(Some(&row));
            }
        }
    }

    pub fn autocomplete_selected(&self) -> bool {
        let _log_guard = logging::function_guard("SearchWindow::autocomplete_selected");
        let results = self.results.blocking_read();
        let index = *self.selected_index.blocking_read();
        if let Some(result) = results.get(index) {
            let current_text = self.entry.text().to_string();
            let completion = result.display_name();

            if completion.starts_with(&current_text) {
                self.entry.set_text(completion);
                self.entry.set_position(-1);
            }

            true
        } else {
            false
        }
    }

    fn configure_window_geometry(window: &ApplicationWindow) {
        let (width, height) = Self::calculate_window_dimensions();
        window.set_default_size(width, height);
    }

    fn calculate_window_dimensions() -> (i32, i32) {
        Self::primary_monitor_geometry()
            .map(|geometry| {
                (
                    (geometry.width() / 2).max(1),
                    (geometry.height() / 2).max(1),
                )
            })
            .unwrap_or((DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
    }

    fn primary_monitor_geometry() -> Option<Rectangle> {
        let display = Display::default()?;
        let monitors = display.monitors();
        let monitor_object = monitors.item(0)?;
        let monitor = monitor_object.downcast::<Monitor>().ok()?;
        Some(monitor.geometry())
    }
}
