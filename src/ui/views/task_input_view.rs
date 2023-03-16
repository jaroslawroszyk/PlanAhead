use super::*;

pub struct TaskInputView;
impl<B: Backend> View<B> for TaskInputView {
    fn post_render(&self, f: &mut Frame<B>, app: &App, _: &mut StatefulUi) {
        let [_, _, footer_chunk] = DefaultView::layout(f.size());

        DefaultView::clear(f, footer_chunk);
        Self::render_footer(f, footer_chunk, &app.input);
    }
}

impl TaskInputView {
    fn render_footer(f: &mut Frame<impl Backend>, area: Rect, text: &str) {
        let text = format!(">> {text}");
        f.render_widget(Paragraph::new(text), area);
    }
}
