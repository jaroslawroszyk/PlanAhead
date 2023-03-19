use super::*;

pub struct TaskInputView;
impl<B: Backend> View<B> for TaskInputView {
    fn post_render(&self, f: &mut Frame<B>, app: &App, _: &mut StatefulUi) {
        let layout = ViewLayout::new(f.size());
        if let Some(footer) = layout.footer {
            ViewLayout::clear(f, footer);
            Self::render_footer(f, footer, &app.input);
        }
    }
}

impl TaskInputView {
    fn render_footer(f: &mut Frame<impl Backend>, area: Rect, text: &str) {
        let text = format!(">> {text}");
        f.render_widget(Paragraph::new(text), area);
    }
}
