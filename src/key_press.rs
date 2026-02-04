use crate::history::ChatSummary;
use crate::history_panel::HistoryPanelState;
use gdk4::{Key, ModifierType};
use glib::{MainContext, Propagation};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Entry, EventControllerKey, Label, ListBox, ListBoxRow, Stack, TextView};
use std::sync::Arc;
use tokio::sync::RwLock;

const NEW_CHAT_ROW_HEADER: &str = "New chat";
const NEW_CHAT_ROW_SUMMARY: &str =
    "Send input while this row is selected to create a fresh AI conversation.";

pub fn history_list_key_controller(
    history_state: Arc<RwLock<HistoryPanelState>>,
    list_box: ListBox,
    stack: Stack,
    placeholder: Label,
    detail: TextView,
) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        if keyval == Key::Delete && state.contains(ModifierType::CONTROL_MASK) {
            schedule_delete_selected_chat(
                history_state.clone(),
                list_box.clone(),
                stack.clone(),
                placeholder.clone(),
                detail.clone(),
            );
            return Propagation::Stop;
        }
        Propagation::Proceed
    });
    controller
}

pub fn history_navigation_controller(
    entry: Entry,
    content_stack: Stack,
    history_state: Arc<RwLock<HistoryPanelState>>,
    list_box: ListBox,
    stack: Stack,
    placeholder: Label,
    detail: TextView,
) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        if !is_history_panel_visible(&content_stack) {
            return Propagation::Proceed;
        }

        if schedule_history_navigation(
            keyval,
            history_state.clone(),
            list_box.clone(),
            stack.clone(),
            placeholder.clone(),
            detail.clone(),
        ) {
            entry.grab_focus();
            return Propagation::Stop;
        }

        if keyval == Key::Delete && state.contains(ModifierType::CONTROL_MASK) {
            schedule_delete_selected_chat(
                history_state.clone(),
                list_box.clone(),
                stack.clone(),
                placeholder.clone(),
                detail.clone(),
            );
            return Propagation::Stop;
        }

        Propagation::Proceed
    });
    controller
}

pub fn sync_history_widgets(
    list_box: &ListBox,
    stack: &Stack,
    placeholder: &Label,
    detail: &TextView,
    state: &HistoryPanelState,
) {
    while let Some(row) = list_box.row_at_index(0) {
        list_box.remove(&row);
    }

    if state.is_empty() {
        stack.set_visible_child_name("history-empty");
        let message = state.empty_state_message();
        placeholder.set_text(message);
        let buffer = detail.buffer();
        buffer.set_text(message);
        return;
    }

    stack.set_visible_child_name("history-list");
    for summary in state.chat_summaries() {
        let row = build_history_row(&summary);
        list_box.append(&row);
    }

    if state.has_new_chat_row() {
        let row = build_new_chat_row();
        list_box.append(&row);
    }

    if let Some(index) = state.selected_index() {
        if let Some(row) = list_box.row_at_index(index as i32) {
            list_box.select_row(Some(&row));
            let buffer = detail.buffer();
            buffer.set_text(&state.selected_timeline_text());
        }
    } else {
        list_box.unselect_all();
        let buffer = detail.buffer();
        buffer.set_text(state.empty_state_message());
    }
}

fn build_history_row(summary: &ChatSummary) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_css_classes(&["history-entry-row"]);
    let container = GtkBox::new(gtk4::Orientation::Vertical, 4);
    container.set_css_classes(&["history-entry-container"]);
    container.set_margin_top(4);
    container.set_margin_bottom(4);
    container.set_margin_start(6);
    container.set_margin_end(6);

    let header = Label::new(Some(&summary.display_label));
    header.set_xalign(0.0);
    header.set_wrap(true);
    header.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
    header.set_css_classes(&["history-entry-header"]);
    container.append(&header);

    let content_label = Label::new(Some(&format!(
        "{} • {} rounds",
        summary.first_round_summary, summary.round_count
    )));
    content_label.set_xalign(0.0);
    content_label.set_wrap(true);
    content_label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
    content_label.set_css_classes(&["history-entry-content"]);
    container.append(&content_label);

    row.set_child(Some(&container));
    row.set_selectable(true);
    row
}

fn build_new_chat_row() -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_css_classes(&["history-entry-row", "history-entry-new"]);
    let container = GtkBox::new(gtk4::Orientation::Vertical, 4);
    container.set_css_classes(&["history-entry-container"]);
    container.set_margin_top(4);
    container.set_margin_bottom(4);
    container.set_margin_start(6);
    container.set_margin_end(6);

    let header = Label::new(Some(NEW_CHAT_ROW_HEADER));
    header.set_xalign(0.0);
    header.set_wrap(true);
    header.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
    header.set_css_classes(&["history-entry-header"]);
    container.append(&header);

    let content_label = Label::new(Some(NEW_CHAT_ROW_SUMMARY));
    content_label.set_xalign(0.0);
    content_label.set_wrap(true);
    content_label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
    content_label.set_css_classes(&["history-entry-content"]);
    container.append(&content_label);

    row.set_child(Some(&container));
    row.set_selectable(true);
    row
}

fn schedule_delete_selected_chat(
    history_state: Arc<RwLock<HistoryPanelState>>,
    list_box: ListBox,
    stack: Stack,
    placeholder: Label,
    detail: TextView,
) {
    MainContext::default().spawn_local(async move {
        let mut guard = history_state.write().await;
        guard.delete_selected_chat();
        drop(guard);
        let guard = history_state.read().await;
        sync_history_widgets(&list_box, &stack, &placeholder, &detail, &guard);
    });
}

fn schedule_history_navigation(
    keyval: Key,
    history_state: Arc<RwLock<HistoryPanelState>>,
    list_box: ListBox,
    stack: Stack,
    placeholder: Label,
    detail: TextView,
) -> bool {
    let handled = matches!(keyval, Key::Up | Key::Down | Key::KP_Up | Key::KP_Down);
    if !handled {
        return false;
    }

    MainContext::default().spawn_local(async move {
        let mut guard = history_state.write().await;
        match keyval {
            Key::Up | Key::KP_Up => guard.move_selection_up(),
            Key::Down | Key::KP_Down => guard.move_selection_down(),
            _ => {}
        }
        drop(guard);
        let guard = history_state.read().await;
        sync_history_widgets(&list_box, &stack, &placeholder, &detail, &guard);
    });

    true
}

fn is_history_panel_visible(stack: &Stack) -> bool {
    stack.visible_child_name().as_deref() == Some("history-panel")
}
