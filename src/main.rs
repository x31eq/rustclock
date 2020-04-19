use chrono::{Local, Datelike, Timelike};
use num_integer::{div_mod_floor, mod_floor};

struct Time {
    quarter: i32,
    week: u8,
    halfday: u8,
    hour: u8,
    ticks: u8,
}

impl Time {
    fn now() -> Self {
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

fn main() {
    let now = Time::now();
    print!("{:03x}{:01x}", mod_floor(now.quarter, 0x1000), now.week);
    println!(".{:01x}{:01x}{:02x}", now.halfday, now.hour, now.ticks);
}
