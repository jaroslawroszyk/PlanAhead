use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn default_view<B: Backend>(f: &mut Frame<B>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" PlanAhead ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(98), Constraint::Percentage(2)].as_ref())
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[0]);

    tasks_view(f, top_chunks[0]);
    calendar_view(f, top_chunks[1]);
    footer_view(f, chunks[1]);
}

fn tasks_view<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title(" Tasks ")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, area);
}

fn calendar_view<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title(" Calendar ")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, area);
}

fn footer_view<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let text = "Q - quit | A - add task | E - edit task | M - modify task | Enter - complete task | ↑/↓ - navigate | D - delete quest";
    f.render_widget(Paragraph::new(text), area);
}