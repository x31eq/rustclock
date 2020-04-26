use rustclock::Time;
use std::env;

fn main() {
    let args = env::args().skip(1);
    if args.len() == 0 {
        eprintln!("Give the timestamp to decode on the command line");
    }
    for festamp in args {
        let result = Time::from_festamp(&festamp).decode();
        println!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            result.tm_year + 1900,
            result.tm_mon + 1,
            result.tm_mday,
            result.tm_hour,
            result.tm_min,
            result.tm_sec,
        )
    }
}
