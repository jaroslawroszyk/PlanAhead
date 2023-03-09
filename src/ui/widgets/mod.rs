pub mod calendar;
pub mod day;

pub use calendar::*;
pub use day::*;

use chrono::{DateTime, Datelike, Local, NaiveDate};
use itertools::Itertools;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Widget},
};
