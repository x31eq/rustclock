use num_integer::mod_floor;
use rustclock::Time;
use std::env;
use time;

fn main() {
    let mut args = env::args().skip(1);
    let instant = if let Some(date_part) = args.next() {
        Time::from_tm(if let Some(time_part) = args.next() {
            time::strptime(
                &format!("{} {}", date_part, time_part),
                "%Y-%m-%d %H:%M:%S",
            )
            .expect("Bad time format")
        } else if date_part.find(' ') == None {
            time::strptime(&date_part, "%Y-%m-%d").expect("Bad date format")
        } else {
            time::strptime(&date_part, "%Y-%m-%d %H:%M:%S")
                .expect("Bad date format")
        })
    } else {
        Time::now()
    };
    print!(
        "{:02x}{:x}",
        mod_floor(instant.quarter, 0x100),
        instant.week
    );
    println!(
        "{:x}:{:x}{:02x}{:x}",
        instant.halfday, instant.hour, instant.tick, instant.sec
    );
}
