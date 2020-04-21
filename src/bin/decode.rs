use rustclock::Time;
use std::env;

fn main() {
    if let Some(stamp) = env::args().skip(1).next() {
        let constructor = {
            if stamp.find(':') == None {
                Time::from_festamp
            } else {
                Time::from_feestamp
            }
        };
        let result = constructor(stamp).decode();
        println!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            result.tm_year + 1900,
            result.tm_mon,
            result.tm_mday,
            result.tm_hour,
            result.tm_min,
            result.tm_sec
        )
    } else {
        eprintln!("Give the timestamp to decode on the command line");
    }
}
