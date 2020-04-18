extern crate chrono;

use chrono::{Datelike, Timelike};

fn main() {
    let local = chrono::Local::now();
    let month = local.month0();
    let quarter = local.year() as u32 * 4 + month / 3;
    let weekday = local.weekday().num_days_from_sunday();
    let qday = month % 3 + (if month == 2 || month == 11 { 1 } else { 0 });
    let week = (qday + local.day() + 5 - weekday) / 7;
    print!("{:x}{:01x}", quarter % 0x1000, week);
    let (pm, hour) = local.hour12();
    let halfday = weekday * 2 + (if pm { 1 } else { 0 });
    let ticks = (local.minute() * 4 + local.second() / 15) * 16 / 15;
    println!(".{:01x}{:01x}{:02x}", halfday, hour, ticks);
}
