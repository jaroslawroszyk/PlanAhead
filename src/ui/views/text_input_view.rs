use super::*;

pub struct TextInputView;
impl<B: Backend> View<B> for TextInputView {
    fn post_render(&self, f: &mut Frame<B>, app: &App, ui: &mut StatefulUi) {
        let layout = ViewLayout::new(f.size());
        if let Some(footer) = layout.footer {
            ViewLayout::clear(f, footer);
            Self::render_footer(f, footer, &app.input, ui);
        }
    }
}

impl TextInputView {
    fn render_footer(f: &mut Frame<impl Backend>, area: Rect, text: &str, ui: &mut StatefulUi) {
        let indicator = ">> ";
        let len = indicator.len() + text.len().saturating_sub(ui.cursor_offset);

        let text: String = if len > area.width as usize {
            text.chars().skip(len - area.width as usize).collect()
        } else {
            text.into()
        };

        let cursor_pos = indicator.len() + text.len().saturating_sub(ui.cursor_offset);
        f.set_cursor(area.x + cursor_pos as u16, area.y);
        f.render_widget(Paragraph::new(format!("{indicator}{text}")), area);
    }
}
