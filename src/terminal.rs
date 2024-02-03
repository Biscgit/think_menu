use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use std::{error::Error, io::stdout};

use crate::types::Terminal;

pub fn setup_terminal() -> Result<Terminal, Box<dyn Error>> {
    // set up terminal for application view and key-binds
    initialize_panic_handler();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Terminal) -> Result<(), Box<dyn Error>> {
    // restore terminal to original state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

pub fn initialize_panic_handler() {
    // cleanup if code panics
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        execute!(std::io::stderr(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}
