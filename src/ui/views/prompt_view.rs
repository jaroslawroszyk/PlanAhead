use super::*;

pub struct PromptView;
impl<B: Backend> View<B> for PromptView {
    fn post_render(&self, f: &mut Frame<B>, _: &App, _: &mut StatefulUi) {
        let txt = "Are you sure ?";
        PromptView::render_prompt(f, txt, 32, 6);
    }
}

impl PromptView {
    fn render_prompt<B: Backend>(f: &mut Frame<B>, prompt_text: &str, width: u16, heigh: u16) {
        let padding_h = (f.size().height - heigh) / 2;
        let padding_w = (f.size().width - width) / 2;

        let chunk_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(padding_h),
                    Constraint::Length(heigh),
                    Constraint::Length(padding_h),
                ]
                .as_ref(),
            )
            .split(f.size());

        let chunk_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(padding_w),
                    Constraint::Length(width),
                    Constraint::Length(padding_w),
                ]
                .as_ref(),
            )
            .split(chunk_vertical[1]);

        let block = Block::default()
            .title(format!(" {prompt_text} "))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from("No - Esc, Yes - Enter"),
        ];
        let parag = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        ViewLayout::clear(f, chunk_horizontal[1]);
        f.render_widget(parag, chunk_horizontal[1]);
    }
}
