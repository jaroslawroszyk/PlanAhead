use super::*;

pub struct DefaultView;
impl<B: Backend> View<B> for DefaultView {}

impl DefaultView {
    pub fn layout(size: Rect) -> [Rect; 3] {
        let [_padding, top, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(size.height - FOOTER_HEIGHT - 1), Constraint::Length(FOOTER_HEIGHT)])
            .split(size)[..3] else { unreachable!() };

        let [top_l, top_r, _padding] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(size.width - CALENDAR_WIDTH), Constraint::Length(CALENDAR_WIDTH-1), Constraint::Length(1)])
            .split(top)[..3] else { unreachable!() };

        #[rustfmt::skip]
        let layout = [
            top_l.inner( &Margin { vertical: 0, horizontal: 2 }),
            top_r.inner( &Margin { vertical: 0, horizontal: 1 }),
            bottom.inner(&Margin { vertical: 1, horizontal: 2 }),
        ];
        layout
    }

    pub fn clear(f: &mut Frame<impl Backend>, area: Rect) {
        let (w, h) = (area.width as usize, area.height as usize);
        let text = vec![Spans::from(Span::styled(" ".repeat(w), Style::reset())); h];
        f.render_widget(Paragraph::new(text), area);
    }

    pub fn render_footer(f: &mut Frame<impl Backend>, area: Rect) {
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

    pub fn render_calendar(f: &mut Frame<impl Backend>, area: Rect) {
        let calendar = CalendarWidget::from(Local::now());
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
            y: area.height - FOOTER_HEIGHT,
            width: area.width - 2,
            height: 1,
        };
        f.render_widget(Block::default().borders(Borders::TOP), area);
    }

    pub fn render_vertical_separator(f: &mut Frame<impl Backend>, area: Rect) {
        let area = Rect {
            x: area.width - CALENDAR_WIDTH,
            y: 1,
            width: 1,
            height: area.height - FOOTER_HEIGHT + 1,
        };
        f.render_widget(Block::default().borders(Borders::RIGHT), area);
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
