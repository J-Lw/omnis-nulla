use crate::ui::ui::{screen_transition, screen_transition_on_enter, WELCOME_MESSAGE};
use anyhow::{Context, Result};
use tracing::{info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

// Initialize the logger with the given log level.
// Returns a subscriber with the max level set to log_level and sets it as the global default.
// Returns an Err value with context if the logger cannot be set.
fn initialize_logger(log_level: Level) -> Result<()> {
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    set_global_default(subscriber).context("Setting default logger failed.")?;
    Ok(())
}

// Initializes the user interface by performing screen transitions and displaying a welcome message.
// Returns an Err value with context if any part of the process fails.
fn initialize_ui() -> Result<()> {
    screen_transition().context("Screen transition failed.")?;
    println!("{}", WELCOME_MESSAGE);
    screen_transition_on_enter().context("Screen transition on enter failed.")?;
    Ok(())
}

// Initializes the program by setting up the logger and user interface.
// Logs a ready message and returns Ok(()) for success and returns an Err with context in cases of failure.
pub fn initialize() -> Result<()> {
    initialize_logger(Level::INFO)?;
    initialize_ui()?;

    info!("Ready.");
    Ok(())
}
