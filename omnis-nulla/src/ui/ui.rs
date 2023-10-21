use anyhow::{Context, Result};
use std::io::{self, BufRead};
use std::process::Command;

pub const WELCOME_MESSAGE: &str = "Welcome, J.";
const ENTER_PROMPT: &str = "\nHit enter.";
const UI_HEADER: &str = r#"
_______                  _____            _____   __      ___________       
__  __ \______ _____________(_)_______    ___  | / /___  ____  /__  /_____ _
_  / / /_  __ `__ \_  __ \_  /__  ___/    __   |/ /_  / / /_  /__  /_  __ `/
/ /_/ /_  / / / / /  / / /  / _(__  )     _  /|  / / /_/ /_  / _  / / /_/ / 
\____/ /_/ /_/ /_//_/ /_//_/  /____/      /_/ |_/  \__,_/ /_/  /_/  \__,_/  
                                                                            
"#;

// Clears the terminal screen.
// May not function properly on non-unix platforms.
// Returns an Err value with context if the command cannot be started or if it fails to execute successfully.
// Else, returns Ok(()).
fn clear_terminal() -> Result<()> {
    Command::new("clear")
        .status()
        .context("Failed to clear the terminal")?;
    Ok(())
}

// Pauses the program until the user presses the enter key.
// Reads a line from standard input and discards it.
// Returns an Err if reading from stdin fails.
fn wait_for_enter() -> Result<()> {
    io::stdin()
        .lock()
        .read_line(&mut String::new())
        .context("Failed to read from stdin")?;
    Ok(())
}

// Clears the terminal and then displays the UI header, effectively performing a screen transition.
// Returns an Err if a problem is encountered during the execution of any task.
pub fn screen_transition() -> Result<()> {
    clear_terminal()?;
    println!("{}", UI_HEADER);
    Ok(())
}

// Prints an enter prompt, pauses the program until the user hits enter, and then performs a screen transition.
// Returns an Err if a problem is encountered during the execution of any task.
pub fn screen_transition_on_enter() -> Result<()> {
    println!("{}", ENTER_PROMPT);
    wait_for_enter()?;
    screen_transition()?;
    Ok(())
}
