use num_integer::mod_floor;
use rustclock::Time;
use std::env;
use time;

fn main() {
    let args = env::args().skip(1);
    let instant = if args.len() == 0 {
        Time::now()
    } else {
        Time::from_tm({
            let datetime = args.collect::<Vec<String>>().join(" ");
            if datetime.find('-') == None {
                time::strptime(
                    &format!("1984-01-01 {}", datetime),
                    "%Y-%m-%d %H:%M:%S",
                )
                .expect("Bad time format")
            } else if datetime.find(' ') == None {
                time::strptime(&datetime, "%Y-%m-%d")
                    .expect("Bad date format")
            } else {
                time::strptime(&datetime, "%Y-%m-%d %H:%M:%S")
                    .expect("Bad date format")
            }
        })
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
