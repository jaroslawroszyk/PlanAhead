use crate::application::{App, InputMode, Task};
use crate::state::states::StatefulList;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn select_view<B: Backend>(input_mode: InputMode) -> Box<dyn View<B>> {
    match input_mode {
        InputMode::AddTask => Box::new(AddTaskView) as Box<dyn View<B>>,
        InputMode::Command => Box::new(DefaultView) as Box<dyn View<B>>,
    }
}

pub trait View<B: Backend> {
    fn render(&self, f: &mut Frame<B>, app: &mut App);
}

struct DefaultView;
impl<B: Backend> View<B> for DefaultView {
    fn render(&self, f: &mut Frame<B>, app: &mut App) {
        let (task_chunk, calendar_chunk, footer_chunk) = Self::layout(f.size());
        Self::render_main(f, f.size());
        Self::render_tasks(f, task_chunk, &mut app.state.tasks);
        Self::render_calendar(f, calendar_chunk);
        Self::render_footer(f, footer_chunk);
    }
}

impl DefaultView {
    fn layout(size: Rect) -> (Rect, Rect, Rect) {
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

    fn render_footer(f: &mut Frame<impl Backend>, area: Rect) {
        let text = "  Q - quit | A - add task | E - edit task | D - delete task | X - clear all tasks | Enter - complete task | ↑/↓ - navigate | ← - unselect ";
        f.render_widget(Paragraph::new(text), area);
    }

    fn render_calendar(f: &mut Frame<impl Backend>, area: Rect) {
        let block = Block::default()
            .title(" Calendar ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        f.render_widget(block, area);
    }

    fn render_tasks(f: &mut Frame<impl Backend>, area: Rect, tasks: &mut StatefulList<Task>) {
        let items: Vec<ListItem> = tasks
            .items
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
        f.render_stateful_widget(list, area, &mut tasks.state)
    }

    fn render_main(f: &mut Frame<impl Backend>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" PlanAhead ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, area);
    }
}

struct AddTaskView;
impl<B: Backend> View<B> for AddTaskView {
    fn render(&self, f: &mut Frame<B>, app: &mut App) {
        let (task_chunk, calendar_chunk, footer_chunk) = DefaultView::layout(f.size());
        DefaultView::render_main(f, f.size());
        DefaultView::render_tasks(f, task_chunk, &mut app.state.tasks);
        DefaultView::render_calendar(f, calendar_chunk);
        Self::render_footer(f, footer_chunk, &app.text_input);
    }
}

impl AddTaskView {
    fn render_footer(f: &mut Frame<impl Backend>, area: Rect, text: &str) {
        let text = format!("  >> {text}");
        f.render_widget(Paragraph::new(text), area);
    }
}
