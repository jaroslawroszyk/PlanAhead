use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{stdout, Stdout},
    ops::{Deref, DerefMut},
};
use tui::backend::CrosstermBackend;

pub struct Terminal(tui::Terminal<CrosstermBackend<Stdout>>);

impl Terminal {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = tui::Terminal::new(backend)?;
        Ok(Terminal(terminal))
    }

    fn restore(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.0.backend_mut(), LeaveAlternateScreen)?;
        self.show_cursor()?;
        Ok(())
    }
}

impl Deref for Terminal {
    type Target = tui::Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.restore().unwrap_or_else(|err| {
            eprintln!("Faild to restore terminal to previous state: {err}");
        });
    }
}
