use ratatui::backend::CrosstermBackend;
use std::{error::Error, io::Stdout};

// often used types
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
