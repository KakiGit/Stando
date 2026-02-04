use crate::history_panel::{HistoryPanelState, DEFAULT_EMPTY_MESSAGE};
use crate::key_press;
use crate::logging;
use crate::search::SearchResult;
use adw::Application;
use anyhow::{Context, Result};
use gdk4::prelude::*;
use gdk4::{Display, Key, Monitor, Rectangle};
use gio::prelude::ListModelExt;
use glib::{prelude::Cast, ControlFlow, Propagation};
use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Box, Button, ButtonsType, DialogFlags, Entry, Label, ListBox, ListBoxRow,
    MessageDialog, MessageType, Paned, ScrolledWindow, Stack, TextView,
};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
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
    content_stack: Stack,
    results_list_box: ListBox,
    history_list_stack: Stack,
    history_list_box: ListBox,
    history_detail: TextView,
    history_empty_label: Label,
    results: Arc<RwLock<Vec<SearchResult>>>,
    selected_index: Arc<RwLock<usize>>,
}

impl SearchWindow {
    fn create_main_box() -> Box {
        let main_box = Box::new(gtk4::Orientation::Vertical, 0);
        main_box.add_css_class("search-main");
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);
        main_box
    }

    fn create_search_box() -> Box {
        let search_box = Box::new(gtk4::Orientation::Horizontal, 8);
        search_box.add_css_class("search-bar");
        search_box.set_margin_start(16);
        search_box.set_margin_end(16);
        search_box.set_margin_top(16);
        search_box.set_margin_bottom(8);
        search_box
    }

    fn create_entry() -> Entry {
        let entry = Entry::new();
        entry.set_placeholder_text(Some(DEFAULT_PLACEHOLDER));
        entry.set_hexpand(true);
        entry.set_margin_start(0);
        entry.set_margin_end(0);
        entry.set_css_classes(&["search-entry"]);
        entry
    }

    fn create_ai_button(ai_shortcut: &str) -> Button {
        let ai_button = Button::new();
        ai_button.set_css_classes(&["ai-mode-button", "search-ai-button"]);
        ai_button.set_tooltip_text(Some(&format!("Toggle AI Mode ({})", ai_shortcut)));
        ai_button.set_label("AI");
        ai_button.set_valign(gtk4::Align::Center);
        ai_button
    }

    fn create_content_stack() -> Stack {
        let content_stack = Stack::new();
        content_stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
        content_stack.set_transition_duration(200);
        content_stack.add_css_class("search-results-stack");
        content_stack.set_hhomogeneous(true);
        // Allow the visible child to dictate height instead of the largest natural size.
        content_stack.set_vhomogeneous(false);
        content_stack.set_vexpand(true);
        content_stack.set_hexpand(true);
        content_stack
    }

    fn create_results_list_box() -> ListBox {
        let results_list_box = ListBox::new();
        results_list_box.set_css_classes(&["search-results-list"]);
        results_list_box.set_selection_mode(gtk4::SelectionMode::Single);
        results_list_box.set_hexpand(true);
        results_list_box.set_vexpand(false);
        results_list_box.set_valign(gtk4::Align::Start);
        results_list_box.set_size_request(-1, 1);
        results_list_box.set_focusable(false);
        results_list_box
    }

    fn create_results_scrolled(
        default_window_height: i32,
        results_list_box: &ListBox,
    ) -> ScrolledWindow {
        let results_scrolled = ScrolledWindow::new();
        results_scrolled.set_css_classes(&["search-results-scroll"]);
        results_scrolled.set_hexpand(true);
        results_scrolled.set_vexpand(true);
        // Keep the window height stable by letting the scroller use available space
        // instead of requesting the list's natural height.
        results_scrolled.set_propagate_natural_height(false);
        results_scrolled.set_propagate_natural_width(false);
        results_scrolled.set_min_content_height(1);
        results_scrolled.set_min_content_width(1);
        results_scrolled.set_max_content_height(default_window_height);
        results_scrolled.set_child(Some(results_list_box));
        results_scrolled
    }

    fn create_history_paned(default_window_width: i32) -> Paned {
        let history_paned = Paned::new(gtk4::Orientation::Horizontal);
        history_paned.set_css_classes(&["history-pane"]);
        let history_detail_position = ((default_window_width * 3) / 4).max(1);
        history_paned.set_position(history_detail_position);
        history_paned
    }

    fn create_history_detail() -> TextView {
        let history_detail = TextView::new();
        history_detail.set_editable(false);
        history_detail.set_cursor_visible(false);
        history_detail.set_wrap_mode(gtk4::WrapMode::WordChar);
        history_detail.set_margin_start(12);
        history_detail.set_margin_end(12);
        history_detail.set_margin_top(12);
        history_detail.set_margin_bottom(12);
        history_detail.set_css_classes(&["history-detail"]);
        history_detail
    }

    fn create_history_list_stack() -> Stack {
        let history_list_stack = Stack::new();
        history_list_stack.add_css_class("history-list-stack");
        history_list_stack.set_vexpand(true);
        history_list_stack.set_hexpand(true);
        history_list_stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        history_list_stack
    }

    fn create_history_list_scrolled() -> ScrolledWindow {
        let history_list_scrolled = ScrolledWindow::new();
        history_list_scrolled.set_hexpand(true);
        history_list_scrolled.set_vexpand(true);
        history_list_scrolled.set_css_classes(&["history-list-scroll"]);
        history_list_scrolled
    }

    fn create_history_list_box() -> ListBox {
        let history_list_box = ListBox::new();
        history_list_box.set_css_classes(&["history-results-list"]);
        history_list_box.set_selection_mode(gtk4::SelectionMode::Single);
        // Keep list non-focusable so focus stays on the entry; key handling is on the entry.
        history_list_box.set_focusable(false);
        history_list_box
    }

    fn create_history_empty_label() -> Label {
        let history_empty_label = Label::new(Some(DEFAULT_EMPTY_MESSAGE));
        history_empty_label.set_wrap(true);
        history_empty_label.set_wrap(true);
        history_empty_label.set_margin_start(12);
        history_empty_label.set_margin_end(12);
        history_empty_label.set_margin_top(16);
        history_empty_label.set_margin_bottom(16);
        history_empty_label.set_css_classes(&["history-empty-label"]);
        history_empty_label
    }

    pub fn new(
        app: &Application,
        ai_shortcut: &str,
        history_panel_state: Arc<RwLock<HistoryPanelState>>,
    ) -> Result<Self> {
        let log_guard = logging::function_guard("SearchWindow::new");
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Stando")
            .resizable(true)
            .decorated(true)
            .build();
        window.add_css_class("search-window");

        // Center and size the window before showing it.
        Self::configure_window_geometry(&window);
        window.connect_realize(|window| {
            SearchWindow::configure_window_geometry(window);
        });
        let (default_window_width, default_window_height) =
            Self::calculate_window_dimensions(Some(&window));

        // Main container
        let main_box = Self::create_main_box();
        window.set_child(Some(&main_box));

        // Search bar container (horizontal box for entry + button)
        let search_box = Self::create_search_box();
        main_box.append(&search_box);

        // Search entry
        let entry = Self::create_entry();
        search_box.append(&entry);

        // AI mode toggle button
        let ai_button = Self::create_ai_button(ai_shortcut);
        search_box.append(&ai_button);

        // Content stack (switch between search results and history panel)
        let content_stack = Self::create_content_stack();
        main_box.append(&content_stack);

        // Search results view
        let results_list_box = Self::create_results_list_box();
        let results_scrolled =
            Self::create_results_scrolled(default_window_height, &results_list_box);
        content_stack.add_named(&results_scrolled, Some("search-results"));

        // History panel view
        let history_paned = Self::create_history_paned(default_window_width);
        let history_detail = Self::create_history_detail();
        let history_list_stack = Self::create_history_list_stack();
        let history_list_scrolled = Self::create_history_list_scrolled();
        let history_list_box = Self::create_history_list_box();
        // Add a key controller to the history list box so that Ctrl+Del
        // deletes the selected chat when the list has focus.
        // History list box and its container are set up above.
        // The empty label is created below.
        let history_empty_label = Self::create_history_empty_label();
        history_list_stack.add_named(&history_empty_label, Some("history-empty"));
        // Place the history list inside the scrolled window and add to stack.
        history_list_scrolled.set_child(Some(&history_list_box));
        history_list_stack.add_named(&history_list_scrolled, Some("history-list"));
        // Add key controller for Ctrl+Del on the history list box.
        let history_list_key_controller = key_press::history_list_key_controller(
            history_panel_state.clone(),
            history_list_box.clone(),
            history_list_stack.clone(),
            history_empty_label.clone(),
            history_detail.clone(),
        );
        history_list_box.add_controller(history_list_key_controller.clone());

        history_paned.set_start_child(Some(&history_detail));
        history_paned.set_end_child(Some(&history_list_stack));
        content_stack.add_named(&history_paned, Some("history-panel"));
        content_stack.set_visible_child_name("search-results");

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

        // Debug widget sizes to diagnose layout constraints.
        Self::attach_size_logger(&window, "window");
        Self::attach_size_logger(&main_box, "main_box");
        Self::attach_size_logger(&search_box, "search_box");
        Self::attach_size_logger(&entry, "entry");
        Self::attach_size_logger(&ai_button, "ai_button");
        Self::attach_size_logger(&content_stack, "content_stack");
        Self::attach_size_logger(&results_scrolled, "results_scrolled");
        Self::attach_size_logger(&results_list_box, "results_list_box");
        Self::attach_size_logger(&history_paned, "history_paned");
        Self::attach_size_logger(&history_detail, "history_detail");
        Self::attach_size_logger(&history_list_stack, "history_list_stack");
        Self::attach_size_logger(&history_list_scrolled, "history_list_scrolled");
        Self::attach_size_logger(&history_list_box, "history_list_box");

        // History interaction helpers
        let history_selection_state = history_panel_state.clone();
        let history_list_box_for_selection = history_list_box.clone();
        let history_list_stack_for_selection = history_list_stack.clone();
        let history_empty_label_for_selection = history_empty_label.clone();
        let history_detail_for_selection = history_detail.clone();
        let history_entry_for_focus = entry.clone();

        history_list_box.connect_row_selected(move |_, row| {
            let entry_index = row.and_then(|row| {
                let index = row.index();
                if index < 0 {
                    None
                } else {
                    Some(index as usize)
                }
            });
            let entry_index = match entry_index {
                Some(index) => index,
                None => return,
            };
            let history_state = history_selection_state.clone();
            let list_box = history_list_box_for_selection.clone();
            let stack = history_list_stack_for_selection.clone();
            let placeholder = history_empty_label_for_selection.clone();
            let detail = history_detail_for_selection.clone();
            let entry_focus = history_entry_for_focus.clone();
            glib::MainContext::default().spawn_local(async move {
                let mut guard = history_state.write().await;
                if guard.is_new_chat_row_index(entry_index) {
                    if guard.is_new_chat_selected() {
                        return;
                    }
                    guard.select_new_chat();
                } else if let Some(chat_id) = guard.chat_id_at_index(entry_index) {
                    if guard.selected_chat_id() == Some(chat_id.as_str())
                        && !guard.is_new_chat_selected()
                    {
                        return;
                    }
                    guard.select_chat(Some(chat_id.clone()));
                    tracing::debug!(selected_chat = %chat_id, "History row selected");
                } else {
                    return;
                }
                drop(guard);
                let guard = history_state.read().await;
                key_press::sync_history_widgets(&list_box, &stack, &placeholder, &detail, &guard);
                entry_focus.grab_focus();
            });
        });

        let navigation_controller = key_press::history_navigation_controller(
            entry.clone(),
            content_stack.clone(),
            history_panel_state.clone(),
            history_list_box.clone(),
            history_list_stack.clone(),
            history_empty_label.clone(),
            history_detail.clone(),
        );
        entry.add_controller(navigation_controller.clone());

        // Initially hidden
        // window.set_visible(false);

        let results = Arc::new(RwLock::new(Vec::new()));
        let selected_index = Arc::new(RwLock::new(0));

        let result = Ok(Self {
            window,
            entry,
            ai_button,
            ai_shortcut: ai_shortcut.to_string(),
            content_stack,
            results_list_box,
            history_list_stack,
            history_list_box,
            history_detail,
            history_empty_label,
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

    /// Returns true when the history panel is the visible content (e.g. AI mode).
    /// Used so key handlers can delegate Up/Down and Ctrl+Del to history navigation when appropriate.
    pub fn is_history_panel_visible(&self) -> bool {
        self.content_stack
            .visible_child_name()
            .as_deref()
            == Some("history-panel")
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
            self.content_stack.set_visible_child_name("history-panel");
        } else {
            self.ai_button.remove_css_class("ai-mode-active");
            self.ai_button.set_tooltip_text(Some(&format!(
                "AI Mode: OFF (Click to enable, Shortcut: {})",
                self.ai_shortcut
            )));
            self.entry.set_placeholder_text(Some(DEFAULT_PLACEHOLDER));
            self.content_stack.set_visible_child_name("search-results");
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
        // Re-apply sizing on each show in case the window manager cached a prior size.
        self.window.unfullscreen();
        self.window.unmaximize();
        Self::configure_window_geometry(&self.window);
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

    pub fn apply_history_state(&self, state: &HistoryPanelState) {
        let _log_guard = logging::function_guard("SearchWindow::apply_history_state");
        key_press::sync_history_widgets(
            &self.history_list_box,
            &self.history_list_stack,
            &self.history_empty_label,
            &self.history_detail,
            state,
        );
    }

    pub fn show_ai_error_notification(&self, message: &str) {
        let dialog = MessageDialog::new(
            Some(&self.window),
            DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT,
            MessageType::Error,
            ButtonsType::Ok,
            message,
        );
        dialog.connect_response(|dialog, _| dialog.close());
        dialog.present();
    }

    pub async fn update_results(&self, new_results: Vec<SearchResult>) {
        let _log_guard = logging::function_guard("SearchWindow::update_results");
        *self.results.write().await = new_results.clone();

        // Clear existing rows
        while let Some(row) = self.results_list_box.row_at_index(0) {
            self.results_list_box.remove(&row);
        }

        // Add new results
        for result in new_results.iter() {
            let row = ListBoxRow::new();
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
            self.results_list_box.append(&row);
        }

        // Select first item
        if let Some(first_row) = self.results_list_box.row_at_index(0) {
            self.results_list_box.select_row(Some(&first_row));
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
        self.results_list_box.connect_row_activated(move |_, _| {
            callback();
        });
    }

    pub fn connect_entry_activate<F: Fn() + 'static>(&self, callback: F) {
        let _log_guard = logging::function_guard("SearchWindow::connect_entry_activate");
        self.entry.connect_activate(move |_| {
            callback();
        });
    }

    pub fn connect_key_press<F: Fn(Key) -> bool + 'static>(&self, callback: F) {
        let _log_guard = logging::function_guard("SearchWindow::connect_key_press");
        let controller = gtk4::EventControllerKey::new();
        controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
            let handled = match keyval {
                Key::Escape
                | Key::Up
                | Key::Down
                | Key::KP_Up
                | Key::KP_Down
                | Key::Tab
                | Key::ISO_Left_Tab => callback(keyval),
                _ => false,
            };

            if handled {
                Propagation::Stop
            } else {
                Propagation::Proceed
            }
        });
        self.entry.add_controller(controller);
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
            if let Some(row) = self.results_list_box.row_at_index((current + 1) as i32) {
                self.results_list_box.select_row(Some(&row));
            }
        }
    }

    pub fn move_selection_up(&self) {
        let _log_guard = logging::function_guard("SearchWindow::move_selection_up");
        // Move selection up in list
        let current = *self.selected_index.blocking_read();
        if current > 0 {
            *self.selected_index.blocking_write() = current - 1;
            if let Some(row) = self.results_list_box.row_at_index((current - 1) as i32) {
                self.results_list_box.select_row(Some(&row));
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
        let (width, height) = Self::calculate_window_dimensions(Some(window));
        window.set_default_size(width, height);
        window.set_resizable(true);
    }

    fn calculate_window_dimensions(window: Option<&ApplicationWindow>) -> (i32, i32) {
        Self::primary_monitor_geometry(window)
            .map(|(geometry, scale)| {
                let monitor_width = geometry.width().max(1);
                let monitor_height = geometry.height().max(1);
                let width = (monitor_width / 2).max(1);
                let height = (monitor_height / 2).max(1);
                tracing::info!(
                    monitor_width = geometry.width(),
                    monitor_height = geometry.height(),
                    monitor_scale = scale,
                    window_width = width,
                    window_height = height,
                    "Computed default window size from monitor geometry"
                );
                (width, height)
            })
            .unwrap_or_else(|| {
                tracing::info!(
                    fallback_width = DEFAULT_WINDOW_WIDTH,
                    fallback_height = DEFAULT_WINDOW_HEIGHT,
                    "Falling back to default window size"
                );
                (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
            })
    }

    fn primary_monitor_geometry(window: Option<&ApplicationWindow>) -> Option<(Rectangle, i32)> {
        if let Some(window) = window {
            if let Some(surface) = window.surface() {
                let display = surface.display();
                if let Some(monitor) = display.monitor_at_surface(&surface) {
                    return Some((monitor.geometry(), monitor.scale_factor()));
                }
            }
        }

        let display = Display::default()?;
        let monitors = display.monitors();
        let monitor_object = monitors.item(0)?;
        let monitor = monitor_object.downcast::<Monitor>().ok()?;
        Some((monitor.geometry(), monitor.scale_factor()))
    }

    fn attach_size_logger<W: gtk4::prelude::IsA<gtk4::Widget>>(widget: &W, name: &'static str) {
        let widget = widget.clone().upcast::<gtk4::Widget>();
        let last = Rc::new(RefCell::new((-1, -1)));
        let last_for_signal = last.clone();
        widget.add_tick_callback(move |widget, _| {
            let width = widget.allocated_width();
            let height = widget.allocated_height();
            let mut last = last_for_signal.borrow_mut();
            if *last != (width, height) {
                *last = (width, height);
                tracing::debug!(widget = name, width, height, "Widget size allocation");
            }
            ControlFlow::Continue
        });
    }
}
