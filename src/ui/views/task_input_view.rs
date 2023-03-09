use super::*;

pub struct TaskInputView;
impl<B: Backend> View<B> for TaskInputView {
    fn render(&self, f: &mut Frame<B>, app: &App, ui: &mut StatefulUi) {
        let (task_chunk, calendar_chunk, footer_chunk) = DefaultView::layout(f.size());
        DefaultView::render_main(f, f.size());
        DefaultView::render_tasks(f, task_chunk, &app.tasks, ui);
        DefaultView::render_calendar(f, calendar_chunk);
        Self::render_footer(f, footer_chunk, &app.input);
    }
}

impl TaskInputView {
    fn render_footer(f: &mut Frame<impl Backend>, area: Rect, text: &str) {
        let text = format!("  >> {text}");
        f.render_widget(Paragraph::new(text), area);
    }
}
