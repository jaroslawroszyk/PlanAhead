use crate::{
    application::event_handlers::*,
    ui::{DefaultView, PromptView, TaskInputView, View},
};
use tui::backend::Backend;

#[derive(Default, Clone, Copy)]
pub enum State {
    #[default]
    Default,
    AddTask,
    EditTask,
    Prompt,
}

impl State {
    pub fn view<B: Backend>(&self) -> Box<dyn View<B>> {
        match self {
            State::AddTask => Box::new(TaskInputView) as Box<dyn View<B>>,
            State::EditTask => Box::new(TaskInputView) as Box<dyn View<B>>,
            State::Default => Box::new(DefaultView) as Box<dyn View<B>>,
            State::Prompt => Box::new(PromptView) as Box<dyn View<B>>,
        }
    }

    pub fn event_handler(&self) -> Box<dyn EventHandler> {
        match self {
            State::AddTask => Box::new(AddTaskEventHandler) as Box<dyn EventHandler>,
            State::EditTask => Box::new(EditTaskEventHandler) as Box<dyn EventHandler>,
            State::Default => Box::new(DefaultEventHandler) as Box<dyn EventHandler>,
            State::Prompt => Box::new(PromptEventHandler) as Box<dyn EventHandler>,
        }
    }
}
