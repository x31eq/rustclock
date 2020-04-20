use num_integer::{div_floor, div_mod_floor, mod_floor};
use time;

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

    pub fn decode(self: Time) -> time::Tm {
        let (year, quarter) = div_mod_floor(self.quarter, 4);
        let year = 1920 + mod_floor(year + 128, 1024);
        let month =
            quarter * 3 + (self.week * 16 + self.halfday) as i32 / 0x55;
        let k = (month % 3) * 38 + 5 - (month == 2 || month == 11) as i32;
        let day = (self.week * 7 + self.halfday / 2) as i32 - k
            + (1 + k - month_weekday(year, month)) % 7;
        let toc = self.tick / 16 * 15 + self.tick % 16;
        time::Tm {
            tm_year: year - 1900,
            tm_mon: month,
            tm_mday: day as i32,
            tm_wday: (self.halfday / 2) as i32,
            tm_hour: (self.hour + 12 * (self.halfday & 1)) as i32,
            tm_min: toc as i32 / 4,
            tm_sec: (toc % 4) as i32 * 15 + self.sec as i32,
            tm_nsec: 0,
            tm_yday: -1,   // not known
            tm_isdst: -1,  // not known
            tm_utcoff: -1, // not known
        }
    }
}

/// Weekday (Sunday is 0) of the first day of the month
/// month is 0 for January
fn month_weekday(year: i32, month: i32) -> i32 {
    // Based on RFC 3339 Appendix B
    let mut y = year;
    let mut m = month - 1;
    if m < 1 {
        m += 12;
        y -= 1;
    }
    let (cent, y) = div_mod_floor(y, 100);
    let day =
        (26 * m - 2) / 10 + 1 + y + y / 4 + div_floor(cent, 4) + 5 * cent;
    mod_floor(day, 7)
}
