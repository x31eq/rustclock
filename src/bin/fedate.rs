use rustclock::Time;
use std::env;

fn main() {
    if let Some(festamp) = env::args().skip(1).next() {
        let mut parts = festamp.split('.');
        let date_part = parts.next().unwrap();
        let time_part = parts.next().unwrap_or("0");
        let dstamp = if date_part.len() > 0 {
            i32::from_str_radix(date_part, 16).expect("Bad date format")
        } else {
            0
        };
        let mut tstamp = if time_part.len() > 0 {
            u32::from_str_radix(time_part, 16).expect("Bad time format")
        } else {
            0
        };
        tstamp <<= 4 * (4 - time_part.len());
        let then = Time {
            quarter: dstamp / 0x10,
            week: dstamp as u8 & 0xf,
            halfday: (tstamp / 0x1000) as u8,
            hour: (tstamp / 0x100) as u8 & 0xf,
            tick: (tstamp & 0xffff) as u8,
            sec: 0,
        };
        let result = then.decode();
        println!("{}-{:02}-{:02} {:02}:{:02}:{:02}",
                 result.tm_year + 1900, result.tm_mon, result.tm_mday,
                 result.tm_hour, result.tm_min, result.tm_sec)
    } else {
        eprintln!("Give the timestamp to decode on the command line");
    }
}
