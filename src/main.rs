use std::io;

use crossterm::terminal::enable_raw_mode;
use tui::{terminal::Terminal, backend::CrosstermBackend};

fn main() {
  enable_raw_mode().expect("can run in raw mode");
  let backend = CrosstermBackend::new(io::stdout());
  let mut terminal = match Terminal::new(backend) {
    Ok(terminal) => terminal,
    Err(err) => {
      eprintln!("Failed to create terminal: {}", err);
      return;
    }
  };
  terminal.clear().expect("failed to clear terminal");
}
