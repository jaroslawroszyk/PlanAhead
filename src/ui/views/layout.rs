use super::*;

macro_rules! r {
    ($x: expr, $y: expr, $w: expr, $h: expr) => {
        Rect {
            x: $x,
            y: $y,
            width: $w,
            height: $h,
        }
    };
}

pub struct ViewLayout {
    pub calendar: Option<Rect>,
    pub footer: Option<Rect>,
    pub tasks: Rect,
}

impl ViewLayout {
    #[rustfmt::skip]
    pub fn new(Rect {x, y, width: w, height: h}: Rect) -> ViewLayout {
        let (mw, mh) = (2,  1); // (margin   width, margin  height)
        let (pw, ph) = (1,  1); // (padding  width, padding height)
        let (cw, fh) = (31, 1); // (calendar width, footer  height)

        match (w, h) {
            (62.., 6..) => ViewLayout {
                calendar: Some(r!(x + w - cw - pw, y + mh          , cw                      , h - 2 * mh - fh - ph   )),
                footer:   Some(r!(x + mw         , y + h - (2 * mh), w.saturating_sub(2 * mw), fh                     )),
                tasks:         r!(x + mw         , y + mh          , w - cw - (2 * mw) - pw  , h - 2 * mh - fh - ph   ),
            },
            (..=61, 6..) => ViewLayout {
                calendar: None,
                footer:   Some(r!(x + mw         , y + h - (2 * mh), w.saturating_sub(2 * mw), fh                     )),
                tasks:         r!(x + mw         , y + mh          , w.saturating_sub(2 * mw), h - 2 * mh - fh - ph   ),
            },
            (62.., ..=5) => ViewLayout {
                calendar: Some(r!(x + w - cw - pw, y + mh          , cw                      , h.saturating_sub(2 * mh))),
                footer:   None,
                tasks:         r!(x + mw         , y + mh          , w - cw - (2 * mw) - pw  , h.saturating_sub(2 * mh)),
            },
            (..=61, ..=5) => ViewLayout {
                calendar: None,
                footer:   None,
                tasks:         r!(x + mw         , y + mh          , w.saturating_sub(2 * mw), h.saturating_sub(2 * mh)),
            },
        }
    }

    pub fn clear(f: &mut Frame<impl Backend>, area: Rect) {
        let (w, h) = (area.width as usize, area.height as usize);
        let text = vec![Spans::from(Span::styled(" ".repeat(w), Style::reset())); h];
        f.render_widget(Paragraph::new(text), area);
    }

    pub fn with_borders(mut self) -> ViewLayout {
        let (mw, mh) = (2, 1); // (margin   width, margin  height)
        let (pw, ph) = (1, 1); // (margin   width, margin  height)

        self.tasks = Rect {
            x: self.tasks.x - mw,
            y: self.tasks.y - mh,
            width: self.tasks.width + 2 * mw,
            height: self.tasks.height + 2 * mh,
        };

        if let Some(calendar) = &mut self.calendar {
            *calendar = Rect {
                x: calendar.x - pw,
                y: calendar.y - ph,
                width: calendar.width + 2 * pw,
                height: calendar.height + 2 * ph,
            };
        }

        if let Some(footer) = &mut self.footer {
            *footer = Rect {
                x: footer.x - mw,
                y: footer.y - mh,
                width: footer.width + 2 * mw,
                height: footer.height + 2 * mh,
            };
        }
        self
    }
}
