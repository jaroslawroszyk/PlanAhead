use crate::app::{App, InputMode};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use dialoguer::{console::style, Confirm};

pub struct EventHandler;

impl EventHandler {
    pub fn on_key(key: KeyEvent, app: &mut App) -> Result<()> {
        if key.kind == KeyEventKind::Press {
            match app.input_mode {
                InputMode::Command => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        app.is_running = false
                    }
                    KeyCode::Char('d') => {
                        app.state.remove_selected_task();
                    }
                    KeyCode::Char('x') => {
                        //Todo: add prompt for user ("Are you sure?...")
                        let confirmed = Confirm::new()
                            .with_prompt(format!(
                                "{} {}",
                                style("Are you sure you want to clear all tasks?").bold(),
                                style("(y/n)").dim()
                            ))
                            .interact()?;
                        if confirmed {
                            app.state.clear_all_tasks()
                        }
                    }
                    KeyCode::Up => app.state.tasks.previous(),
                    KeyCode::Down => app.state.tasks.next(),
                    KeyCode::Left => app.state.tasks.unselect(),
                    KeyCode::Char('a') | KeyCode::Char('A') => app.input_mode = InputMode::AddTask,
                    _ => (),
                },
                InputMode::AddTask => match key.code {
                    KeyCode::Enter => {
                        app.state.add_task();
                        app.input_mode = InputMode::Command;
                    }
                    KeyCode::Char(c) => {
                        app.state.text_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.state.text_input.pop();
                    }
                    KeyCode::Esc => app.input_mode = InputMode::Command,
                    _ => (),
                },
            }
        }
        Ok(())
    }
}
