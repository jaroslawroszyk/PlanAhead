use super::*;

pub struct CalendarView;
impl<B: Backend> View<B> for CalendarView {
    fn post_render(&self, f: &mut Frame<B>, _: &App, _: &mut StatefulUi) {
        let layout = ViewLayout::new(f.size()).with_borders();

        if let Some(calendar) = layout.calendar {
            DefaultView::highligh_area(f, calendar);
        }
    }
}
