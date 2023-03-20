use super::*;
use chrono::Month;
use num_traits::FromPrimitive;

pub struct CalendarWidget {
    year: i32,
    month: u32,
    selected: u32,
}

impl Default for CalendarWidget {
    fn default() -> Self {
        let now = Local::now();
        CalendarWidget {
            year: now.year(),
            month: now.month(),
            selected: now.day(),
        }
    }
}

impl Widget for CalendarWidget {
    fn render(self, mut area: Rect, buf: &mut Buffer) {
        let bounds = self.widget_bounds(4, 2);
        if area.width < bounds.width || area.height < bounds.height {
            return; // TODO: figure out what to do when area is to small
        };
        area = self.align_area(area, bounds);
        let [header, arrows, weekdays, rest] = self.layout(area);

        let month = Month::from_u32(self.month).unwrap();
        Paragraph::new(format!(" {} {}", month.name(), self.year)).render(header, buf);
        Paragraph::new("▲  ▼").render(arrows, buf);
        Paragraph::new(" Mon Tue Wed Thu Fri Sat Sun").render(weekdays, buf);

        let days = self.generate_day_numbers();
        let grid = self.day_grid(rest);

        for ((day, curr_month), area) in days.zip(grid) {
            let style = match (curr_month, day == self.selected) {
                (true, true) => Style::default().fg(Color::Black).bg(Color::White),
                (true, false) => Style::default(),
                (false, _) => Style::default().fg(Color::Gray),
            };
            DayWidget { day, style }.render(area, buf);
        }
    }
}

impl CalendarWidget {
    #[rustfmt::skip]
    fn layout(&self, area: Rect) -> [Rect; 4] {
        let header   = Rect { x: 0,            y: 0, width: area.width - 4, height: 1  };
        let arrows   = Rect { x: header.width, y: 0, width: 4,              height: 1  };
        let weekdays = Rect { x: 0,            y: 2, width: area.width,     height: 1  };
        let days     = Rect { x: 0,            y: 3, width: area.width,     height: area.height - 3 };

        [header, arrows, weekdays, days].map(|r| Rect {
            x: r.x + area.x,
            y: r.y + area.y,
            width: r.width,
            height: r.height,
        })
    }

    fn day_grid(&self, area: Rect) -> impl Iterator<Item = Rect> {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(area.width / 7); 7])
            .split(area);

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(area.height / 6); 6])
            .split(area)
            .into_iter();

        rows.cartesian_product(cols).map(|(row, col)| Rect {
            x: col.x,
            y: row.y,
            width: col.width,
            height: row.height,
        })
    }

    fn widget_bounds(&self, day_width: u16, day_height: u16) -> Rect {
        let header_height = 3;
        Rect {
            x: 0,
            y: 0,
            width: day_width * 7,
            height: day_height * 6 + header_height,
        }
    }

    fn align_area(&self, mut area: Rect, bounds: Rect) -> Rect {
        let padd = (area.width - bounds.width) / 2;
        area.width = bounds.width;
        area.x += padd;
        area.height = bounds.height;
        area
    }

    fn generate_day_numbers(&self) -> impl Iterator<Item = (u32, bool)> {
        let curr = NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap();
        let offset = self.days_offset();

        let curr_month_day_count = self.next().signed_duration_since(curr).num_days() as u32 + 1;
        let prev_month_day_count = curr.signed_duration_since(self.prev()).num_days() as u32 + 1;
        let next_month_days_left = 6 * 7 - (offset + curr_month_day_count - 1) + 1;

        let prev_days = (prev_month_day_count - offset..prev_month_day_count).map(|n| (n, false));
        let curr_days = (1..curr_month_day_count).map(|n| (n, true));
        let next_days = (1..next_month_days_left).map(|n| (n, false));
        prev_days.chain(curr_days).chain(next_days)
    }

    fn prev(&self) -> NaiveDate {
        match self.month {
            1 => NaiveDate::from_ymd_opt(self.year - 1, 12, 1).unwrap(),
            _ => NaiveDate::from_ymd_opt(self.year, self.month - 1, 1).unwrap(),
        }
    }

    fn next(&self) -> NaiveDate {
        match self.month {
            12 => NaiveDate::from_ymd_opt(self.year + 1, 1, 1).unwrap(),
            _ => NaiveDate::from_ymd_opt(self.year, self.month + 1, 1).unwrap(),
        }
    }

    fn days_offset(&self) -> u32 {
        let first = NaiveDate::from_ymd_opt(self.year, self.month, 1);
        let first_weekday = first.unwrap().weekday();
        first_weekday.num_days_from_monday()
    }
}

impl From<DateTime<Local>> for CalendarWidget {
    fn from(date: DateTime<Local>) -> Self {
        CalendarWidget {
            year: date.year(),
            month: date.month(),
            selected: date.day(),
        }
    }
}
