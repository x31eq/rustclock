/// Current time in bfee format (no leap seconds)

use time;
use num_integer::div_mod_floor;

pub struct Time {
    pub quarter: i32,
    pub week: u8,
    pub halfday: u8,
    pub hour: u8,
    pub tick: u8,
    pub sec: u8,
}

impl Time {
    pub fn now() -> Self {
        let local = time::now();
        let month = local.tm_mon;
        let (quarter, month3) = div_mod_floor(month, 3);
        let weekday = local.tm_wday;
        let qday = month3 * 38 - (month == 2 || month == 11) as i32;
        let (pm, hour) = div_mod_floor(local.tm_hour, 12);
        let leap_second = local.tm_sec / 60;
        let (tick, sec) = div_mod_floor(local.tm_sec - leap_second, 15);
        Time {
            quarter: (local.tm_year + 1900) * 4 + quarter as i32,
            week: ((qday + local.tm_mday + 5 - weekday) / 7) as u8,
            halfday: weekday as u8 * 2 + pm as u8,
            hour: hour as u8,
            tick: ((local.tm_min * 4 + tick) * 16 / 15) as u8,
            sec: (sec + leap_second) as u8,
        }
    }
}
