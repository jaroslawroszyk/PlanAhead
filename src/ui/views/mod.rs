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

const FOOTER_HEIGHT: u16 = 3;
const CALENDAR_WIDTH: u16 = 31;

pub trait View<B: Backend> {
    fn render(&self, f: &mut Frame<B>, app: &App, ui: &mut StatefulUi) {
        let window = f.size();
        if window.height < 4 || window.width < 8 {
            return;
        }
        let [mut top_left, top_right, bottom] = DefaultView::layout(window);

        match (
            window.width > 2 * CALENDAR_WIDTH,
            window.height > 2 * FOOTER_HEIGHT,
        ) {
            (true, true) => {
                DefaultView::render_calendar(f, top_right);
                DefaultView::render_vertical_separator(f, window);
                DefaultView::render_footer(f, bottom);
                DefaultView::render_horizontal_separator(f, window);
            }
            (false, true) => {
                top_left.width = window.width - 2 * top_left.x;
                DefaultView::render_footer(f, bottom);
                DefaultView::render_horizontal_separator(f, window);
            }
            (true, false) => {
                top_left.height = window.height - 2 * top_left.y;
                DefaultView::render_vertical_separator(f, window);
                DefaultView::render_calendar(f, top_right);
            }
            (false, false) => {
                top_left.height = window.height - 2 * top_left.y;
                top_left.width = window.width - 2 * top_left.x;
            }
        }
        DefaultView::render_tasks(f, top_left, &app.tasks, ui);
        DefaultView::render_main(f, window);
        self.post_render(f, app, ui)
    }

    fn post_render(&self, _: &mut Frame<B>, _: &App, _: &mut StatefulUi) {}
}
