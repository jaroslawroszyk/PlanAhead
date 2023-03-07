use anyhow::Result;
use crossterm::event;
use plan_ahead::{application::App, backend::terminal::Terminal, ui::stateful_ui::StatefulUi};

fn main() -> Result<()> {
    let mut terminal = Terminal::new()?;
    let mut app = App::default();
    let mut ui = StatefulUi::default();

    while app.is_running {
        let view = app.state.view();
        terminal.draw(|f| view.render(f, &app, &mut ui))?;

        let handler = app.state.event_handler();
        handler.on_event(event::read()?, &mut app, &mut ui);
    }
    Ok(())
}
