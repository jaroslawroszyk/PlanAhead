mod add_task_view;
mod default_view;
mod prompt_view;

pub use add_task_view::*;
pub use default_view::*;
pub use prompt_view::*;

use crate::{
    application::{App, Task},
    ui::StatefulUi,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub trait View<B: Backend> {
    fn render(&self, f: &mut Frame<B>, app: &App, ui: &mut StatefulUi);
}
