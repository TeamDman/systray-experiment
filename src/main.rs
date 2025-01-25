// #![windows_subsystem = "windows"]

use clap::Parser;
use std::{thread, time::Duration};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Console::GetConsoleWindow;
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOW};

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(version, about = "An experimental application")]
struct Args {
    /// If set, enable debug logging
    #[arg(long)]
    debug: bool,
}

/// Toggles the visibility of the console window
fn toggle_console_visibility(show: bool) {
    unsafe {
        let hwnd: HWND = GetConsoleWindow();
        if hwnd.0.is_null() {
            ShowWindow(hwnd, if show { SW_SHOW } else { SW_HIDE }).expect("should show window");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let args = Args::parse();

    // Setup logging
    let log_level = if args.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let env_filter = EnvFilter::builder()
        .with_default_directive(log_level.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    info!("Starting the application...");

    let mut show_console = true;
    loop {
        // Toggle console visibility
        toggle_console_visibility(show_console);
        show_console = !show_console;

        // Log the state
        if show_console {
            info!("Console is now visible.");
        } else {
            info!("Console is now hidden.");
        }

        // Wait for 3 seconds
        thread::sleep(Duration::from_secs(3));
    }
}
