mod add_task_handler;
mod default_handler;
mod edit_task_handler;
mod prompt_handler;

pub use add_task_handler::*;
pub use default_handler::*;
pub use edit_task_handler::*;
pub use prompt_handler::*;

use crate::{
    application::{Action, App, State},
    ui::StatefulUi,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub trait EventHandler {
    fn on_event(&self, event: Event, app: &mut App, ui: &mut StatefulUi) {
        if let Event::Key(key) = event {
            self.on_key(key, app, ui)
        }
    }

    fn on_key(&self, key: KeyEvent, app: &mut App, ui: &mut StatefulUi) {
        if KeyEventKind::Press == key.kind {
            self.on_key_press(key, app, ui)
        }
    }

    fn on_key_press(&self, key: KeyEvent, app: &mut App, ui: &mut StatefulUi);
}
