/// Current time in bfee format (no leap seconds)

use chrono::{Local, Datelike, Timelike};
use num_integer::div_mod_floor;

pub struct Time {
    pub quarter: i32,
    pub week: u8,
    pub halfday: u8,
    pub hour: u8,
    pub ticks: u8,
}

impl Time {
    pub fn now() -> Self {
        let local = Local::now();
        let month = local.month0();
        let (quarter, month3) = div_mod_floor(month, 3);
        let weekday = local.weekday().num_days_from_sunday();
        let qday = month3 + (month == 2 || month == 11) as u32;
        let (pm, hour) = local.hour12();
        Time {
            quarter: local.year() * 4 + quarter as i32,
            week: ((qday + local.day() + 5 - weekday) / 7) as u8,
            halfday: weekday as u8 * 2 + pm as u8,
            hour: hour as u8 % 12,
            ticks: ((local.minute() * 4 + local.second() / 15) * 16 / 15) as u8,
        }
    }
}
