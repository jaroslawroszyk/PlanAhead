pub mod add_task_view;
pub mod default_view;
pub mod prompt_view;

pub use crate::application::{App, InputMode};
pub use add_task_view::*;
pub use default_view::*;
pub use prompt_view::*;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn select_view<B: Backend>(input_mode: InputMode) -> Box<dyn View<B>> {
    match input_mode {
        InputMode::AddTask => Box::new(AddTaskView) as Box<dyn View<B>>,
        InputMode::Command => Box::new(DefaultView) as Box<dyn View<B>>,
        InputMode::Prompt => Box::new(PromptView) as Box<dyn View<B>>,
    }
}

pub trait View<B: Backend> {
    fn render(&self, f: &mut Frame<B>, app: &mut App);
}
