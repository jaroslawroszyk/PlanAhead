use super::*;

#[derive(Default)]
pub struct DayWidget {
    pub day: u32,
    pub style: Style,
}

impl Widget for DayWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = format!(" {:02} ", self.day);
        let paragraph = Paragraph::new(vec![
            Spans::from(Span::styled("", self.style)),
            Spans::from(Span::styled(&text, self.style)),
            Spans::from(Span::styled("", self.style)),
        ]);
        paragraph.render(area, buf);
    }
}
