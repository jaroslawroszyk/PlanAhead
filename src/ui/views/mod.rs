mod default_view;
mod layout;
mod prompt_view;
mod task_input_view;

pub use default_view::*;
pub use layout::*;
pub use prompt_view::*;
pub use task_input_view::*;

use crate::{
    application::{App, Task},
    ui::{views::ViewLayout, CalendarWidget, StatefulUi},
};
use chrono::Local;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub trait View<B: Backend> {
    fn render(&self, f: &mut Frame<B>, app: &App, ui: &mut StatefulUi) {
        let layout = ViewLayout::new(f.size());
        DefaultView::render_tasks(f, layout.tasks, &app.tasks, ui);

        if let Some(calendar) = layout.calendar {
            DefaultView::render_calendar(f, calendar);
        }
        if let Some(footer) = layout.footer {
            DefaultView::render_footer(f, footer);
        }

        DefaultView::render_main(f, f.size());
        self.post_render(f, app, ui)
    }

    fn post_render(&self, _: &mut Frame<B>, _: &App, _: &mut StatefulUi) {}
}
