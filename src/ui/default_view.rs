use super::*;
use crate::application::{App, Task};

pub struct DefaultView;
impl<B: Backend> View<B> for DefaultView {
    fn render(&self, f: &mut Frame<B>, app: &App, ui: &mut StatefulUi) {
        let (task_chunk, calendar_chunk, footer_chunk) = Self::layout(f.size());
        Self::render_main(f, f.size());
        Self::render_tasks(f, task_chunk, &app.tasks, ui);
        Self::render_calendar(f, calendar_chunk);
        Self::render_footer(f, footer_chunk);
    }
}

impl DefaultView {
    pub fn layout(size: Rect) -> (Rect, Rect, Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(98), Constraint::Percentage(2)].as_ref())
            .split(size);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(chunks[0]);

        (top_chunks[0], top_chunks[1], chunks[1])
    }

    pub fn render_footer(f: &mut Frame<impl Backend>, area: Rect) {
        let command = "  Q - quit | A - add task | E - edit task | D - delete task | X - clear all tasks | Enter - complete task | ↑/↓ - navigate | ← - unselect ";
        f.render_widget(Paragraph::new(command), area);
    }

    pub fn render_calendar(f: &mut Frame<impl Backend>, area: Rect) {
        let block = Block::default()
            .title(" Calendar ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        f.render_widget(block, area);
    }

    pub fn render_tasks(
        f: &mut Frame<impl Backend>,
        area: Rect,
        tasks: &[Task],
        ui: &mut StatefulUi,
    ) {
        let items: Vec<ListItem> = tasks
            .iter()
            .map(|task| ListItem::new(format!("  {}", task.name)))
            .collect();
        let block = Block::default()
            .title(" Tasks ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
            .highlight_symbol(" >> ");
        f.render_stateful_widget(list, area, &mut ui.tasks)
    }

    pub fn render_main(f: &mut Frame<impl Backend>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" PlanAhead ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, area);
    }
}
