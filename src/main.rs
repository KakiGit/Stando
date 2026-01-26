use adw::Application;
use anyhow::{Context, Result};
use clap::Parser;
use gtk4::prelude::*;
use std::rc::Rc;

mod ai;
mod app;
mod config;
mod history;
mod logging;
mod search;
mod ui;
mod window;

use app::App;

#[derive(Parser, Debug)]
#[command(name = "stando")]
#[command(about = "A next-gen selection based search system for Linux", long_about = None)]
struct Args {
    /// Enable verbose logging (equivalent to RUST_LOG=debug)
    #[arg(short, long)]
    verbose: bool,
    /// Start Stando with AI mode enabled
    #[arg(short = 'a', long)]
    ai_mode: bool,
}

fn main() -> Result<()> {
    let log_guard = logging::function_guard("main");
    let result = (|| {
        let args = Args::parse();

        // Initialize logging
        let filter = if args.verbose {
            tracing_subscriber::EnvFilter::new("debug")
        } else {
            tracing_subscriber::EnvFilter::from_default_env()
        };

        tracing_subscriber::fmt().with_env_filter(filter).init();

        // Initialize GTK
        gtk4::init().context("Failed to initialize GTK")?;

        // Create application
        let application = Application::builder()
            .application_id("com.stando.App")
            .build();

        let ai_mode_enabled = args.ai_mode;
        application.connect_activate(move |app| {
            // Create app instance
            let stando_app = match App::new(app.clone(), ai_mode_enabled) {
                Ok(app) => app,
                Err(e) => {
                    eprintln!("Failed to create application: {}", e);
                    std::process::exit(1);
                }
            };

            // Initialize asynchronously using glib's main context
            let stando_app_for_init = Rc::new(stando_app);
            let stando_app_clone = stando_app_for_init.clone();

            glib::MainContext::default().spawn_local(async move {
                if let Err(e) = stando_app_clone.initialize().await {
                    eprintln!("Failed to initialize application: {}", e);
                }
            });

            // Keep reference alive
            std::mem::forget(stando_app_for_init);
        });

        // Run application with only the program name to prevent GTK from processing our arguments
        let args: Vec<String> = std::env::args().take(1).collect();
        application.run_with_args(&args);
        Ok(())
    })();
    if result.is_err() {
        log_guard.mark_error();
    }
    result
}
