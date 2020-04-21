use rustclock::Time;
use std::env;

fn main() {
    if let Some(feestamp) = env::args().nth(1) {
        let result = Time::from_feestamp(&feestamp).decode();
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
