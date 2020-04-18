use chrono::{Local, Datelike, Timelike};
use num_integer::{div_mod_floor, mod_floor};

fn main() {
    let local = Local::now();
    let month = local.month0();
    let (quarter, month3) = div_mod_floor(month, 3);
    let quarter = mod_floor(local.year(), 1024) as u32 * 4 + quarter;
    let weekday = local.weekday().num_days_from_sunday();
    let qday = month3 + (month == 2 || month == 11) as u32;
    let week = (qday + local.day() + 5 - weekday) / 7;
    print!("{:03x}{:01x}", quarter, week);
    let (pm, hour) = local.hour12();
    let halfday = weekday * 2 + pm as u32;
    let ticks = (local.minute() * 4 + local.second() / 15) * 16 / 15;
    println!(".{:01x}{:01x}{:02x}", halfday, hour, ticks);
}
