use anyhow::Result;
use crossterm::event;
use plan_ahead::{application::App, backend::terminal::Terminal, ui::stateful_ui::StatefulUi};
use std::panic;

fn main() -> Result<()> {
    panic::set_hook(Box::new(panic_hook));

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

fn panic_hook(info: &panic::PanicInfo<'_>) {
    use crossterm::{execute, terminal};

    terminal::disable_raw_mode().unwrap();
    execute!(std::io::stdout(), terminal::LeaveAlternateScreen).unwrap();
    println!("{info}");
}
