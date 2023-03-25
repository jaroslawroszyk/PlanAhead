use super::*;

pub struct DefaultView;
impl<B: Backend> View<B> for DefaultView {
    fn post_render(&self, f: &mut Frame<B>, _: &App, _: &mut StatefulUi) {
        let layout = ViewLayout::new(f.size()).with_borders();
        Self::highligh_area(f, layout.tasks);
    }
}

impl DefaultView {
    pub fn render_footer(f: &mut Frame<impl Backend>, area: Rect) {
        DefaultView::render_horizontal_separator(f, area);
        let dimmed = Style::default().add_modifier(Modifier::DIM);

        #[rustfmt::skip]
        let shortcuts = Paragraph::new(Spans::from(vec![
            Span::raw("Q"),    Span::styled("uit",   dimmed),
            Span::raw(" | A"), Span::styled("dd",    dimmed),
            Span::raw(" | E"), Span::styled("dit",   dimmed),
            Span::raw(" | D"), Span::styled("elete", dimmed),
            Span::raw(" | C"), Span::styled("lear",  dimmed),
        ]));
        let navigation = Paragraph::new("Enter | Esc | Tab | ↑/↓ | ←/→");

        f.render_widget(navigation.alignment(Alignment::Right), area);
        f.render_widget(shortcuts.alignment(Alignment::Left), area);
    }

    pub fn highligh_area(f: &mut Frame<impl Backend>, area: Rect) {
        let highlight = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);
        f.render_widget(highlight, area);

        let app_title = Block::default()
            .title(" PlanAhead ")
            .title_alignment(Alignment::Center);
        f.render_widget(app_title, f.size());
    }

    pub fn render_calendar(f: &mut Frame<impl Backend>, area: Rect, ui: &StatefulUi) {
        DefaultView::render_vertical_separator(f, area);
        let calendar = CalendarWidget::from(ui.date);
        f.render_widget(calendar, area);
    }

    fn make_content(task: &Task) -> Spans {
        let style = match task.is_done {
            true => Style::default().add_modifier(Modifier::CROSSED_OUT),
            false => Style::default(),
        };
        Spans::from(Span::styled(task.name.clone(), style))
    }

    pub fn render_tasks(
        f: &mut Frame<impl Backend>,
        area: Rect,
        tasks: &[Task],
        ui: &mut StatefulUi,
    ) {
        let items: Vec<ListItem> = tasks
            .iter()
            .map(|task| ListItem::new(Self::make_content(task)))
            .collect();

        let list = List::new(items)
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
            .highlight_symbol("* ");

        f.render_stateful_widget(list, area, &mut ui.tasks);
    }

    pub fn render_horizontal_separator(f: &mut Frame<impl Backend>, area: Rect) {
        let area = Rect {
            x: 1,
            y: area.y.saturating_sub(1),
            width: area.width + 2,
            height: 1,
        };
        f.render_widget(Block::default().borders(Borders::TOP), area);
    }

    pub fn render_vertical_separator(f: &mut Frame<impl Backend>, area: Rect) {
        let area = Rect {
            x: area.x.saturating_sub(1),
            y: 1,
            width: 1,
            height: area.height,
        };
        f.render_widget(Block::default().borders(Borders::LEFT), area);
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
