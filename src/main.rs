use anyhow::Result;
use crossterm::event;
use plan_ahead::{
    application::{App, EventHandler},
    backend::terminal::Terminal,
    ui::{self, stateful_ui::StatefulUi},
};

fn main() -> Result<()> {
    let mut terminal = Terminal::new()?;
    let mut app = App::default();
    let mut ui = StatefulUi::default();

    while app.is_running {
        let view = ui::select_view(app.input_mode);
        terminal.draw(|f| view.render(f, &app, &mut ui))?;

        if let event::Event::Key(key) = event::read()? {
            EventHandler::on_key(key, &mut app, &mut ui);
        }
    }
    Ok(())
}
