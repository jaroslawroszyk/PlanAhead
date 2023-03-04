use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::{App, InputMode},
    state::State,
};

pub fn default_view<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

    tasks_view(f, top_chunks[0], &mut app.state);
    calendar_view(f, top_chunks[1]);
    footer_view(f, chunks[1], app);
}

fn tasks_view<B: Backend>(f: &mut Frame<B>, area: Rect, state: &mut State) {
    let items: Vec<ListItem> = state
        .tasks
        .items
        .iter()
        .map(|task| ListItem::new(format!("  {}", &*task.name)))
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
    f.render_stateful_widget(list, area, &mut state.tasks.state)
}

fn calendar_view<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .title(" Calendar ")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, area);
}

fn footer_view<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    match &app.input_mode {
        InputMode::Command => {
            let text = "  Q - quit | A - add task | E - edit task | D - delete task | Enter - complete task | ↑/↓ - navigate | ← - unselect ";
            f.render_widget(Paragraph::new(text), area);
        }
        InputMode::AddTask => {
            let text = format!("  >> {}", app.state.text_input);
            f.render_widget(Paragraph::new(text), area);
        }
    }
}
