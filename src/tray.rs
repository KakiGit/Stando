use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{MenuButton, PopoverMenu};
use gio::prelude::*;
use adw::Application;

pub struct TrayIcon {
    menu_button: MenuButton,
    popover: PopoverMenu,
}

impl TrayIcon {
    pub fn new(app: &Application) -> Result<Self> {
        // Create a menu button that can be used as a tray icon
        // In a real implementation, this would integrate with the system tray
        // For now, we create a minimal window that can be minimized to tray
        
        let menu_button = MenuButton::new();
        
        // Create a menu model
        let menu = gio::Menu::new();
        menu.append(Some("Show"), Some("app.show"));
        menu.append(Some("Quit"), Some("app.quit"));
        
        let popover = PopoverMenu::from_model(Some(&menu));
        menu_button.set_popover(Some(&popover));
        
        // Connect quit action via application action
        let quit_action = gio::SimpleAction::new("quit", None);
        let app_for_quit = app.clone();
        quit_action.connect_activate(move |_, _| {
            app_for_quit.quit();
        });
        app.add_action(&quit_action);
        
        Ok(Self {
            menu_button,
            popover,
        })
    }

    pub fn menu_button(&self) -> &MenuButton {
        &self.menu_button
    }

    pub fn connect_show<F: Fn() + 'static>(&self, callback: F) {
        // Note: Menu actions should be connected via application actions
        // For now, this is a placeholder - the menu is handled via gio::Menu
        // In a full implementation, you'd use gio::SimpleAction and connect to "app.show"
        let _ = callback;
    }

    pub fn show(&self) {
        self.menu_button.set_visible(true);
    }

    pub fn hide(&self) {
        self.menu_button.set_visible(false);
    }
}

// Note: GTK4 removed StatusIcon. For a full system tray implementation:
// 1. Use libappindicator via FFI bindings
// 2. Use GTK4's StatusNotifier (if available)
// 3. Create a minimal always-visible window that acts as tray
// 4. Integrate with desktop environment's tray implementation

// This implementation provides a basic menu that can be integrated
// into the main application window or used as a standalone tray window
