use rustclock::Time;
use std::env;

fn main() {
    let mut found = false;
    for stamp in env::args().skip(1) {
        found = true;
        let constructor = {
            if stamp.find(':') == None {
                Time::from_festamp
            } else {
                Time::from_feestamp
            }
        };
        let result = constructor(&stamp).decode();
        println!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            result.tm_year + 1900,
            result.tm_mon,
            result.tm_mday,
            result.tm_hour,
            result.tm_min,
            result.tm_sec
        )
    }
    if !found {
        eprintln!("Give the timestamp to decode on the command line");
    }
}
