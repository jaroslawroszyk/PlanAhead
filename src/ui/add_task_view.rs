use super::{DefaultView, View};
use crate::application::App;
use tui::{backend::Backend, layout::Rect, widgets::Paragraph, Frame};

pub struct AddTaskView;
impl<B: Backend> View<B> for AddTaskView {
    fn render(&self, f: &mut Frame<B>, app: &mut App) {
        let (task_chunk, calendar_chunk, footer_chunk) = DefaultView::layout(f.size());
        DefaultView::render_main(f, f.size());
        DefaultView::render_tasks(f, task_chunk, &mut app.state.tasks);
        DefaultView::render_calendar(f, calendar_chunk);
        Self::render_footer(f, footer_chunk, &app.text_input);
    }
}

impl AddTaskView {
    fn render_footer(f: &mut Frame<impl Backend>, area: Rect, text: &str) {
        let text = format!("  >> {text}");
        f.render_widget(Paragraph::new(text), area);
    }
}
