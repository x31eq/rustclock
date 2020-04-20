use std::env;
use rustclock::Time;

fn main() {
    if let Some(feestamp) = env::args().skip(1).next() {
        let mut parts = feestamp.split(':');
        let dstamp = parts.next().expect("Timestamp must include a :");
        let tstamp = parts.next().unwrap_or("0");
        let dstamp = i32::from_str_radix(dstamp, 16).expect("Bad format");
        let tstamp = u32::from_str_radix(tstamp, 16).expect("Bad format");
        let then = Time {
                quarter: dstamp / 0x100 + 1984 * 4,
                week: (dstamp / 0x10) as u8 & 0xf,
                halfday: (dstamp & 0xf) as u8,
                hour: (tstamp / 0x1000) as u8,
                tick: ((tstamp / 0x10) & 0xff) as u8,
                sec: (tstamp & 0xf) as u8,
                };
        println!("{}", then.decode().rfc3339());
    }
    else {
        eprintln!("Give the timestamp to decode on the command line");
    }
}
