extern crate chrono;
extern crate num_integer;

use chrono::{Datelike, Timelike};
use num_integer::div_mod_floor;

fn main() {
    let local = chrono::Local::now();
    let month = local.month0();
    let (quarter, month3) = div_mod_floor(month, 3);
    let quarter = local.year() as u32 * 4 + quarter;
    let weekday = local.weekday().num_days_from_sunday();
    let qday = month3 + (if month == 2 || month == 11 { 1 } else { 0 });
    let week = (qday + local.day() + 5 - weekday) / 7;
    print!("{:x}{:01x}", quarter % 0x1000, week);
    let (pm, hour) = local.hour12();
    let halfday = weekday * 2 + (if pm { 1 } else { 0 });
    let ticks = (local.minute() * 4 + local.second() / 15) * 16 / 15;
    println!(".{:01x}{:01x}{:02x}", halfday, hour, ticks);
}
