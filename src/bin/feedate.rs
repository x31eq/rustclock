use rustclock::Time;
use std::env;

fn main() {
    if let Some(feestamp) = env::args().skip(1).next() {
        let mut parts = feestamp.split(':');
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
            quarter: dstamp / 0x100 + 1984 * 4,
            week: (dstamp / 0x10) as u8 & 0xf,
            halfday: (dstamp & 0xf) as u8,
            hour: (tstamp / 0x1000) as u8,
            tick: ((tstamp / 0x10) & 0xff) as u8,
            sec: (tstamp & 0xf) as u8,
        };
        println!("{}", then.decode().strftime("%Y-%m-%d %T").unwrap());
    } else {
        eprintln!("Give the timestamp to decode on the command line");
    }
}
