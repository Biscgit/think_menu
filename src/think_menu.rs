use tokio::io::Result;
use crate::types::Terminal;

pub struct ThinkMenu {}

impl ThinkMenu {
    pub fn new() -> ThinkMenu {
        ThinkMenu {}
    }
    pub async fn run(mut self, terminal: &mut Terminal) -> Result<()> {
        Ok(())
    }
}