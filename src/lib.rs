use num_integer::{div_mod_floor, mod_floor};
use std::env;
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
    pub fn from_tm(local: time::Tm) -> Self {
        let year = local.tm_year + 1900;
        let month = local.tm_mon;
        let (quarter, month3) = div_mod_floor(month, 3);
        // strptime doesn't set tm_wday
        let weekday = weekday(year, month, local.tm_mday);
        let qday = month3 * 38 - (month == 2 || month == 11) as i32;
        let (pm, hour) = div_mod_floor(local.tm_hour, 12);
        let leap_second = local.tm_sec / 60;
        let (tick, sec) = div_mod_floor(local.tm_sec - leap_second, 15);
        Time {
            quarter: year * 4 + quarter as i32,
            week: ((qday + local.tm_mday + 5 - weekday) / 7) as u8,
            halfday: weekday as u8 * 2 + pm as u8,
            hour: hour as u8,
            tick: ((local.tm_min * 4 + tick) * 16 / 15) as u8,
            sec: (sec + leap_second) as u8,
        }
    }

    pub fn now() -> Self {
        Time::from_tm(time::now())
    }

    pub fn decode(self: &Time) -> time::Tm {
        let (year, quarter) = div_mod_floor(self.quarter, 4);
        let month =
            quarter * 3 + (self.week * 16 + self.halfday) as i32 / 0x55;
        // c.f. from_tm
        let qday = (month % 3) * 38 - (month == 2 || month == 11) as i32;
        // week = (qday + day + 5 - weekday) / 7    [1]
        // weekday = (weekday_1 + day - 1) % 7      [2]
        // qday as above
        // day = day of month (first day = 1) (we want to find this)
        // weekday = days since Sunday
        // weekday_1 = days since Sunday for the first day of the month
        //
        // Rearrange [1]
        // week * 7 = qday + day + 5 - weekday
        //            - (qday + day + 5 - weekday) % 7
        // day = week * 7 + weekday - qday - 5
        //       + (qday + day + 5 - weekday) % 7
        //
        // Substitute in [2]
        // day = week * 7 + weekday - qday - 5
        //       + (qday + day + 5 - (weekday_1 + day - 1)) % 7
        // day = week * 7 + weekday - qday - 5 - (qday + 6 - weekday_1) % 7
        let day = (self.week * 7 + self.halfday / 2) as i32 - qday - 5
            + (qday + 6 - weekday(year, month, 1)) % 7;
        let toc = self.tick / 16 * 15 + self.tick % 16;
        time::Tm {
            tm_year: year - 1900,
            tm_mon: month,
            tm_mday: day,
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

    pub fn from_festamp(festamp: &str) -> Self {
        let mut parts = festamp.split('.');
        let date_part = parts.next().unwrap();
        let time_part = parts.next().unwrap_or("0");
        let dstamp = if date_part.is_empty() {
            0
        } else {
            i32::from_str_radix(date_part, 16).expect("Bad week format")
        };
        let mut tstamp = if time_part.is_empty() {
            0
        } else {
            u32::from_str_radix(time_part, 16).expect("Bad time format")
        };
        let mut quarter = dstamp / 0x10;
        if date_part.len() < 4 {
            quarter += epoch_from_env() * 4;
        } else if date_part.len() < 5 {
            quarter += default_hexennium(quarter);
        }
        tstamp <<= 4 * (5 - time_part.len());
        Time {
            quarter,
            week: dstamp as u8 & 0xf,
            halfday: (tstamp / 0x1_0000) as u8,
            hour: ((tstamp / 0x1000) & 0xf) as u8,
            tick: ((tstamp / 0x10) & 0xffff) as u8,
            sec: (tstamp & 0xf) as u8,
        }
    }

    pub fn from_feestamp(feestamp: &str) -> Self {
        let mut parts = feestamp.split(':');
        let date_part = parts.next().unwrap();
        let time_part = parts.next().unwrap_or("0");
        let dstamp = if date_part.is_empty() {
            0
        } else {
            i32::from_str_radix(date_part, 16).expect("Bad date format")
        };
        let mut tstamp = if time_part.is_empty() {
            0
        } else {
            u32::from_str_radix(time_part, 16).expect("Bad time format")
        };
        let mut quarter = dstamp / 0x100;
        if date_part.len() < 5 {
            quarter += epoch_from_env() * 4;
        } else if date_part.len() < 6 {
            quarter += default_hexennium(quarter);
        }
        tstamp <<= 4 * (4 - time_part.len());
        Time {
            quarter,
            week: (dstamp / 0x10) as u8 & 0xf,
            halfday: (dstamp & 0xf) as u8,
            hour: (tstamp / 0x1000) as u8,
            tick: ((tstamp / 0x10) & 0xff) as u8,
            sec: (tstamp & 0xf) as u8,
        }
    }

    /// From command line arguments or current time if there none
    ///
    /// This is a bit ad hoc but used by two binaries
    pub fn from_args() -> Self {
        let datetime = env::args().skip(1).collect::<Vec<String>>().join(" ");
        if datetime.is_empty() {
            Time::now()
        } else {
            Time::from_tm({
                if datetime.find('@') == Some(0) {
                    let mut datetime = datetime;
                    datetime.remove(0);
                    let stamp: i64 = datetime.parse().expect("Bad timestamp");
                    time::at(time::Timespec::new(stamp, 0))
                } else if datetime.find('-') == None {
                    time::strptime(
                        &format!("1984-01-01 {}", datetime),
                        "%Y-%m-%d %T",
                    )
                    .expect("Bad time format")
                } else {
                    [
                        "%Y-%m-%d %T",
                        "%Y-%m-%dT%T",
                        "%Y-%m-%d %H:%M",
                        "%Y-%m-%d",
                    ]
                    .iter()
                    .map(|template| time::strptime(&datetime, template))
                    .find(Result::is_ok)
                    .expect("Bad datetime format")
                    .unwrap()
                }
            })
        }
    }
}

/// Get the epoch from the environment, or default to 1984
fn epoch_from_env() -> i32 {
    if let Ok(epoch_str) = env::var("HEXEPOCH") {
        if let Ok(epoch) = epoch_str.parse::<i32>() {
            return epoch;
        }
        eprintln!("Bad HEXEPOCH {}.  Defaulting to 1984", epoch_str);
    };
    1984
}

/// Decide on the leading digit for a two-digit quarter
fn default_hexennium(quarter: i32) -> i32 {
    // Default hexennium starts with 1920
    if quarter < 0xe00 {
        0x2000
    } else {
        0x1000
    }
}

/// Weekday of the given day (Sunday is 0, January is 0)
fn weekday(year: i32, month: i32, day: i32) -> i32 {
    // Based on RFC 3339 Appendix B
    let mut y = year;
    let mut m = month - 1;
    if m < 1 {
        m += 12;
        y -= 1;
    }
    let (cent, y) = div_mod_floor(y, 100);
    let day = (26 * m - 2) / 10 + day + y + y / 4 + (cent >> 2) + 5 * cent;
    mod_floor(day, 7)
}
