use num_integer::mod_floor;
use rustclock::Time;

fn main() {
    let instant = Time::from_args();
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
