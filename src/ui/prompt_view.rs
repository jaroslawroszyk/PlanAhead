use super::*;
use crate::application::App;

pub struct PromptView;
impl<B: Backend> View<B> for PromptView {
    fn render(&self, f: &mut Frame<B>, app: &mut App) {
        DefaultView {}.render(f, app);
        let txt = "Are you sure ?";
        PromptView::render_prompt(f, txt, 30, 18);
    }
}

impl PromptView {
    fn render_prompt<B: Backend>(f: &mut Frame<B>, prompt_text: &str, width: u16, heigh: u16) {
        let padding_h = (100 - heigh) / 2;
        let padding_w = (100 - width) / 2;

        let chunk_vertical = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(padding_h),
                    Constraint::Percentage(heigh),
                    Constraint::Percentage(padding_h),
                ]
                .as_ref(),
            )
            .split(f.size());

        let chunk_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(padding_w),
                    Constraint::Percentage(width),
                    Constraint::Percentage(padding_w),
                ]
                .as_ref(),
            )
            .split(chunk_vertical[1]);

        let block = Block::default()
            .title(" Prompt ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = vec![
            Spans::from(prompt_text),
            Spans::from(""),
            Spans::from("Esc - Cancel, Yes - Enter"),
        ];
        let parag = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(parag, chunk_horizontal[1]);
    }
}
