pub mod add_task_view;
pub mod default_view;

use add_task_view::*;
use default_view::*;

use crate::application::{App, InputMode};
use tui::{backend::Backend, Frame};

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
