use chrono::{Datelike, Utc};

pub fn get_current_year_and_month() -> (u32, u32) {
    let now = Utc::now();
    (now.year() as u32, now.month() as u32)
}