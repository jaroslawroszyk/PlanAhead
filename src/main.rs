use anyhow::Result;
use crossterm::event;
use plan_ahead::{app::*, event_handler::*, terminal::*, ui};

fn main() -> Result<()> {
    let mut terminal = Terminal::new()?;
    let mut app = App::default();
    app.state.load_tasks()?;

    while app.is_running {
        terminal.draw(|f| ui::default_view(f, &mut app))?;

        if let event::Event::Key(key) = event::read()? {
            EventHandler::on_key(key, &mut app)
        }
    }
    Ok(())
}
