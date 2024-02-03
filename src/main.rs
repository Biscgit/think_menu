use std::{
    error::Error,
    result::Result,
};

use crate::{
    think_menu::ThinkMenu,
    terminal::*,
};

mod terminal;
mod think_menu;
mod types;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut terminal = setup_terminal()?;

    let app = ThinkMenu::new();
    let result = app.run(&mut terminal).await;

    restore_terminal(terminal)?;
    result?;

    Ok(())
}