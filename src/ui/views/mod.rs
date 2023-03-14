mod default_view;
mod prompt_view;
mod task_input_view;

pub use default_view::*;
pub use prompt_view::*;
pub use task_input_view::*;

use crate::{
    application::{App, Task},
    ui::{CalendarWidget, StatefulUi},
};
use chrono::Local;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub trait View<B: Backend> {
    fn render(&self, f: &mut Frame<B>, app: &App, ui: &mut StatefulUi);
}
